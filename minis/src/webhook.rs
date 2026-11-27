//! An optional Discord webhook notice when a run finishes, so the person
//! hosting this build can see that someone used it.
//!
//! Off unless the environment variable `AELSIGN_DISCORD_WEBHOOK` holds a
//! Discord webhook URL. Only non-identifying facts are sent: the app name and
//! version, the iOS version, whether the run succeeded, and the country-level
//! locale the phone reports. Never the UDID, device name, Apple Account,
//! pairing data, certificate, or any IPA content — those belong to the user.
//!
//! The send is fire-and-forget: a slow or failing webhook must never delay or
//! break an install, so errors are swallowed and the request is bounded by a
//! short timeout.

use std::time::Duration;

/// The environment variable holding the webhook URL.
const ENV: &str = "AELSIGN_DISCORD_WEBHOOK";
/// Long enough to leave, short enough never to hold up an install.
const TIMEOUT: Duration = Duration::from_secs(5);

/// Whether a webhook is configured at all, without revealing its URL.
pub fn configured() -> bool {
    std::env::var(ENV).is_ok_and(|url| is_valid(&url))
}

/// A Discord webhook URL must be https on discord.com's api host. Refusing
/// anything else keeps a tampered environment from sending the notice (and
/// any data in it) somewhere unexpected.
fn is_valid(url: &str) -> bool {
    let rest = match url.strip_prefix("https://discord.com/api/webhooks/") {
        Some(rest) => rest,
        None => match url.strip_prefix("https://discordapp.com/api/webhooks/") {
            Some(rest) => rest,
            None => return false,
        },
    };
    // `<id>/<token>`: digits, a slash, then a non-empty token.
    let mut parts = rest.splitn(2, '/');
    let id = parts.next().unwrap_or("");
    let token = parts.next().unwrap_or("");
    !id.is_empty()
        && id.bytes().all(|b| b.is_ascii_digit())
        && !token.is_empty()
        && token.len() >= 20
}

/// What a run reports. Deliberately free of anything identifying.
pub struct Notice<'a> {
    /// The app that was installed, e.g. "ESign" or "AelSign".
    pub app: &'a str,
    /// The app's version, when it has one.
    pub version: Option<&'a str>,
    /// The iPhone's iOS version, e.g. "27.0".
    pub ios: &'a str,
    /// Whether the run installed the app or stopped early.
    pub ok: bool,
    /// How many seconds the run took.
    pub seconds: u64,
}

/// The JSON body for the webhook. Kept pure so it can be tested: it must never
/// carry the UDID, a device name, an Apple Account, or a pairing secret.
fn body(n: &Notice<'_>) -> serde_json::Value {
    let status = if n.ok { "✅ Success" } else { "⚠️ Failed" };
    let title = match n.version {
        Some(v) => format!("{} {v} installed", n.app),
        None => format!("{} installed", n.app),
    };
    let title = if n.ok { title } else { format!("{} attempt", n.app) };
    serde_json::json!({
        "username": "AelSign",
        "embeds": [{
            "title": title,
            "color": if n.ok { 0x4cc38a } else { 0xe5a140 },
            "fields": [
                { "name": "Status", "value": status, "inline": true },
                { "name": "iOS", "value": n.ios, "inline": true },
                { "name": "Took", "value": format!("{}s", n.seconds), "inline": true }
            ],
            "footer": { "text": "AelSign installer" }
        }]
    })
}

/// Send the notice, if a webhook is configured. Never fails the run: a bad URL,
/// a network error, or a slow server is logged quietly and ignored.
pub async fn send(n: Notice<'_>) {
    let Ok(url) = std::env::var(ENV) else {
        return;
    };
    if !is_valid(&url) {
        crate::ui::detail("Discord webhook is not a valid discord.com webhook URL; skipped.");
        return;
    }
    let client = match reqwest::Client::builder().timeout(TIMEOUT).build() {
        Ok(client) => client,
        Err(_) => return,
    };
    // No embeds from user input, so no allowlist concerns; just the shape above.
    let _ = client
        .post(&url)
        .json(&body(&n))
        .send()
        .await
        .map_err(|e| crate::ui::detail(&format!("Discord webhook failed: {e}")));
}

/// The install-time convenience: build the notice and send. Safe to call
/// anywhere; a no-op without a webhook. Never affects the run's result.
pub async fn installed(app_name: &str, version: Option<&str>, ios: &str, started: std::time::Instant) {
    if !configured() {
        return;
    }
    send(Notice {
        app: app_name,
        version,
        ios,
        ok: true,
        seconds: started.elapsed().as_secs(),
    })
    .await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_without_the_environment_variable() {
        // The test process normally has no webhook set; `configured` must not
        // panic and must reflect the variable honestly.
        if std::env::var(ENV).is_err() {
            assert!(!configured());
        }
    }

    #[test]
    fn only_discord_webhook_urls_are_accepted() {
        assert!(is_valid("https://discord.com/api/webhooks/123/ABCdef0123456789abcdef"));
        assert!(is_valid("https://discordapp.com/api/webhooks/1/abcdefghij0123456789"));
        assert!(!is_valid("http://discord.com/api/webhooks/1/abcdefghij0123456789"));
        assert!(!is_valid("https://evil.example/api/webhooks/1/abcdefghij0123456789"));
        assert!(!is_valid("https://discord.com/api/webhooks/abc/abcdefghij0123456789"));
        assert!(!is_valid("https://discord.com/api/webhooks/1/short"));
        assert!(!is_valid(""));
    }

    #[test]
    fn the_notice_carries_no_identifying_data() {
        let n = Notice {
            app: "ESign",
            version: Some("1.0"),
            ios: "27.0",
            ok: true,
            seconds: 42,
        };
        let text = body(&n).to_string();
        assert!(text.contains("ESign"));
        assert!(text.contains("27.0"));
        assert!(!text.to_lowercase().contains("udid"));
        assert!(!text.contains("unknownuserael"));
    }

    #[test]
    fn a_failed_run_says_so() {
        let n = Notice {
            app: "Ksign",
            version: None,
            ios: "26.1",
            ok: false,
            seconds: 3,
        };
        let text = body(&n).to_string();
        assert!(text.contains("Failed") || text.contains("attempt"));
    }
}

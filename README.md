# AelSign installer for Open Minis

This fork of [siboot by FrizzleM](https://github.com/FrizzleM/siboot) adds a local Files picker for choosing an IPA and a live install log in Open Minis. The page is served from the iPhone itself; the chosen IPA is uploaded only to the local installer, not to a public website.

Needs iOS 27+, Developer Mode, Wi-Fi, LocalDevVPN, and Open Minis 1.16+. The app source is [AelSign](https://github.com/Youaremurderinmymind/Xel). Keep the original license and attribution; this fork is unofficial.

The `Build Open Minis installer` GitHub Action builds the ARM64 installer and pairing helper on the `aelsign-web-picker` review branch. After reviewing and merging that branch into `main`, wait for the successful main-branch build before running this in Open Minis:

```sh
wget -qO- https://raw.githubusercontent.com/Youaremurderinmymind/siboot/main/minis/install.sh | sh
```

The first run opens AelSign's local picker. Select an `.ipa`; upload progress and signing/install logs appear in the same Minis page. The action intentionally leaves the installer checksum invalid until the rebuilt binary is ready, so an old upstream binary is refused rather than silently installed.

**Attribution:** based on [siboot by FrizzleM](https://github.com/FrizzleM/siboot).

# AelSign installer for Open Minis

This fork of [siboot by FrizzleM](https://github.com/FrizzleM/siboot) adds a local Files picker for choosing an IPA and a live install log in Open Minis. The page is served from the iPhone itself; the chosen IPA is uploaded only to the local installer, not to a public website.

Needs iOS 27+, Developer Mode, Wi-Fi, LocalDevVPN, and Open Minis 1.16+. The app source is [AelSign](https://github.com/Youaremurderinmymind/Xel). Keep the original license and attribution; this fork is unofficial.

The `Build Open Minis installer` GitHub Action builds the ARM64 installer and pairing helper on the `aelsign-web-picker` review branch. After reviewing and merging that branch into `main`, wait for the successful main-branch build before running this in Open Minis:

```sh
wget -qO- https://raw.githubusercontent.com/Youaremurderinmymind/siboot/main/minis/install.sh | sh
```

The first run opens AelSign's local picker. Select an `.ipa`; upload progress and signing/install logs appear in the same Minis page. To test the review branch before merging, wait for its Action to succeed and run:

```sh
wget -qO- https://raw.githubusercontent.com/Youaremurderinmymind/siboot/aelsign-web-picker/minis/install.sh | SIBOOT_BASE='https://raw.githubusercontent.com/Youaremurderinmymind/siboot/aelsign-web-picker/minis' sh
```

The branch's checksum is populated only by a successful build, so the installer refuses an old binary before then. After reviewing and merging the PR, use the `main` install command above. Choose only IPA files you trust; don't share your Apple Account password or GitHub sign-in codes in chat.

**Attribution:** based on [siboot by FrizzleM](https://github.com/FrizzleM/siboot).

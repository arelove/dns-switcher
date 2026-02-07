<div align="center">

<img src="./docs/images/logo.png" alt="DNS Switcher Logo" width="256"/>

# DNS Switcher

*A lightweight DNS switcher for Windows with system tray and mini app integration* 🌟

Switch between DNS providers instantly with a clean, modern interface.

[![Windows](https://img.shields.io/badge/Windows-10%2F11-0078D6?logo=windows)](https://github.com/arelove/dns-switcher/releases)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![GitHub release](https://img.shields.io/github/v/release/arelove/dns-switcher)](https://github.com/arelove/dns-switcher/releases)
[![GitHub stars](https://img.shields.io/github/stars/arelove/dns-switcher?style=social)](https://github.com/arelove/dns-switcher/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/arelove/dns-switcher?style=social)](https://github.com/arelove/dns-switcher/network/members)
[![GitHub issues](https://img.shields.io/github/issues/arelove/dns-switcher)](https://github.com/arelove/dns-switcher/issues)

[Download](/releases) 📥 • [Features](#features) • [Screenshots](#screenshots) • [Documentation](./docs/) 📚

</div>

## Why DNS Switcher? 🤔

Changing DNS settings manually through Windows network settings is tedious. This app makes it instant! 💨

- **One-click switching** between popular DNS providers (Cloudflare, Google, Quad9, AdGuard, etc.) 🔄
- **Custom presets** for your own DNS servers 🛠️
- **System tray** for quick access 🖥️
- **Mini mode** that stays out of your way 📱
- **Micro mode** - ultra-compact always-on-top widget 🔍
- **Dark/Light theme** support (50/50 split) 🌗

Perfect for developers, privacy-conscious users, or anyone who needs to switch DNS frequently. 🔒

## Screenshots 📸

<div align="center">
<table>
    <tr>
        <td align="center" width="60%">
            <b>Main Window</b><br/>
            <img src="./docs/images/screenshot-main.png" alt="Main Window" width="100%"/>
        </td>
        <td align="center" width="40%">
            <b>Compact Modes</b><br/>
            <img src="./docs/images/screenshot-mini.png" alt="Mini Mode" width="100%"/>
            <br/><br/>
            <img src="./docs/images/screenshot-micro.png" alt="Micro Mode" width="100%"/>
        </td>
    </tr>
</table>
<em>Mini mode for quick access, Micro mode for always-on-top widget</em> ✨
</div>

## Features ✨

**Quick Presets**  
- Pre-configured DNS providers ready to use 🔧  
- Popular services: Cloudflare (1.1.1.1), Google (8.8.8.8), Quad9, OpenDNS, and more 🌐  
- Privacy-focused options like AdGuard DNS 🛡️  

**Custom DNS**  
- Add your own DNS servers ➕  
- Create custom lists for different use cases 📋  
- Filter and search through your presets 🔍  

**Smart Interface**  
- Shows current active DNS 📊  
- Real-time status updates ⏱️  
- Mini mode for compact view 📱  
- Micro mode - ultra-compact widget that stays on top 🖼️  
- Minimal resource usage (~20MB RAM) ⚡  
- Runs in the background via system tray 🔔  

**Developer-Friendly**  
- Built with Tauri (Rust + Web) for speed and security 🦀  
- Small installer size (~5MB) 📦  
- Open source and transparent 👀  

## Download 📥

Grab the latest version from the [Releases](/releases) page.  

**Windows 10/11 Required** | ~5MB installer 💻  

## Quick Start 🏁

1. Download and run the installer 📂  
2. Click a DNS preset to apply it instantly 🔄  
3. Use the system tray icon for quick access 🖥️  
4. Switch to mini mode for a compact view 📱  
5. Or use micro mode for an always-on-top widget 🔍  

That's it—no configuration needed! 🎉  

## Building from Source 🛠️

<details>
<summary>Click to expand build instructions</summary>

### Prerequisites

```bash
# Install Rust 🦀
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js (v18+) 📦
# Download from nodejs.org or use nvm
```

### Steps

```bash
git clone https://github.com/arelove/dns-switcher.git
cd dns-switcher

npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

The installer will be located in:  
`src-tauri/target/release/bundle/`

</details>

## Tech Stack 🔧

- **Frontend**: SvelteKit + TypeScript  
- **Backend**: Rust (via Tauri)  
- **Styling**: Pure CSS (no frameworks)  
- **Build tool**: Vite  

Tauri provides **much smaller binaries**, superior performance, and a truly native experience — far better than Electron in most cases.

## How It Works Under the Hood 🔍

The application uses Windows `netsh` commands to modify DNS settings on each network adapter individually.  
This is why a **UAC (admin rights)** prompt appears on first launch.  
DNS changes are applied **per-adapter** (not globally), which properly respects VPN connections, different Wi-Fi profiles, Ethernet, etc.

## Contributing 🤝

Found a bug or have an idea?

1. Check the existing [issues](https://github.com/arelove/dns-switcher/issues) first  
2. Open a new issue with clear description  
3. Submit pull requests (best after discussing in an issue)

We aim to keep the project **simple**, **fast**, and **focused**.

## Known Limitations ⚠️

- DNS changing requires administrator privileges (Windows limitation)  
- Some VPN software may override applied DNS settings  

## Roadmap 🗺️

Planned / considered features:

- macOS and Linux support  
- Automatic network profile detection (home / office / public)  
- DNS server latency & speed testing  
- Import / export of presets  

Feedback and suggestions are very welcome!

## License 📄

[Apache 2.0](LICENSE) — free to use, modify, and distribute.

Built something cool using this project? Feel free to share — I'd love to hear about it ❤️

**Like the project?**  
Please star it on GitHub ⭐ or share it with anyone annoyed by manual DNS changes!

Questions, bugs, ideas → [Open an issue](https://github.com/arelove/dns-switcher/issues)
```

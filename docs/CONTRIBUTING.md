# Contributing to DNS Switcher

Thanks for considering contributing! This document explains how to set up the development environment and contribute to the project.

## Development Setup

### Prerequisites

**Required:**

- [Rust](https://rustup.rs/) (1.70+)
- [Node.js](https://nodejs.org/) (v18+)
- Windows 10 (build 17763+) or Windows 11
- Visual Studio Build Tools or Visual Studio 2019/2022

**Recommended:**

- [VS Code](https://code.visualstudio.com/) with recommended extensions
- [Git](https://git-scm.com/)

### First Time Setup

1. **Clone the repository**

   ```bash
   git clone https://github.com/YOUR_USERNAME/dns-switcher.git
   cd dns-switcher
   ```

2. **Install Node dependencies**

   ```bash
   npm install
   ```

3. **Install Rust (if not already installed)**

   ```bash
   # Windows
   # Download from https://rustup.rs/
   ```

4. **Open in VS Code**

   ```bash
   code .
   ```

5. **Install recommended extensions**
   - VS Code will prompt you to install recommended extensions
   - Accept to install Svelte, Rust, Prettier, etc.

### Running in Development

```bash
# Run the app in development mode
npm run tauri dev

# The app will launch with hot-reload enabled
# Changes to Svelte files will auto-refresh
# Changes to Rust files will trigger rebuild
```

**Note:** You'll be prompted for admin rights on first launch.

### Building for Production

```bash
# Build optimized version
npm run tauri build

# Output will be in:
# src-tauri/target/release/bundle/
```

---

## Project Structure

```text
dns-switcher/
├── src/                      # Frontend (SvelteKit)
│   ├── lib/
│   │   ├── components/      # Svelte components
│   │   ├── api.ts          # Tauri API calls
│   │   ├── types.ts        # TypeScript types
│   │   └── stores.ts       # Svelte stores
│   └── routes/             # SvelteKit routes
│       ├── +page.svelte    # Main window
│       └── mini/           # Mini window
│
├── src-tauri/               # Backend (Rust + Tauri)
│   ├── src/
│   │   ├── main.rs         # Entry point
│   │   ├── lib.rs          # Tauri commands
│   │   ├── dns.rs          # DNS management logic
│   │   ├── network.rs      # Network adapter detection
│   │   ├── presets.rs      # Built-in DNS presets
│   │   ├── custom_presets.rs # Custom preset storage
│   │   ├── tray.rs         # System tray
│   │   └── types.rs        # Rust types
│   ├── icons/              # App icons
│   ├── Cargo.toml          # Rust dependencies
│   └── tauri.conf.json     # Tauri configuration
│
├── docs/                    # Documentation
│   └── images/             # Screenshots
│
├── package.json            # Node dependencies
├── svelte.config.js        # SvelteKit config
└── vite.config.js          # Vite config
```

---

## Code Style

### TypeScript/Svelte

- **Formatter:** Prettier (auto-formats on save)
- **Style:** See `.prettierrc`
- Use tabs for indentation
- Single quotes for strings
- Trailing commas in ES5

```typescript
// Good
const servers = ['8.8.8.8', '8.8.4.4'];

// Bad
const servers = ["8.8.8.8", "8.8.4.4"]
```

### Rust

- **Formatter:** rustfmt (auto-formats on save)
- Follow standard Rust style guide
- Use descriptive variable names
- Add doc comments for public functions

```rust
/// Sets DNS servers for a network adapter
///
/// # Arguments
/// * `adapter_name` - Name of the network adapter
/// * `ipv4_servers` - List of IPv4 DNS servers
pub fn set_dns(adapter_name: &str, ipv4_servers: Vec<String>) -> Result<()> {
    // implementation
}
```

---

## Making Changes

### Before You Start

1. **Check existing issues** - someone might already be working on it
2. **Open an issue** to discuss the change (for big features)
3. **Fork the repo** and create a branch

### Development Workflow

1. **Create a feature branch**

   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
   - Write clean, readable code
   - Follow existing patterns
   - Test thoroughly

3. **Test locally**

   ```bash
   npm run tauri dev

   ```text
   - Test all affected features
   - Test on different network configurations if possible

4. **Commit your changes**

   ```bash
   git add .
   git commit -m "Add: your feature description"
   ```

   Use conventional commits:
   - `Add:` new feature
   - `Fix:` bug fix
   - `Update:` modify existing feature
   - `Refactor:` code restructuring
   - `Docs:` documentation changes

5. **Push and create PR**

   ```bash
   git push origin feature/your-feature-name
   ```

   Then open a Pull Request on GitHub

---

## What to Contribute

### Good First Issues

Easy ways to contribute:

- Fix typos in documentation
- Improve error messages
- Add more DNS provider presets
- Translate UI to other languages (planned feature)
- Improve comments in code

### Feature Ideas

Things we'd love to see:

- macOS support (requires significant work)
- Linux support (requires significant work)
- Network profile detection (auto-switch DNS by network)
- DNS latency testing improvements
- Import/export settings
- More detailed logging
- Better error recovery

### Bug Fixes

Always welcome:

- Network adapter detection issues
- UI bugs
- Performance problems
- Edge cases

---

## Testing

### Manual Testing Checklist

Before submitting PR, test:

- [ ] App launches without errors
- [ ] Can detect network adapters
- [ ] Can set DNS from presets
- [ ] Can create custom preset
- [ ] Can edit custom preset
- [ ] Can delete custom preset
- [ ] System tray works
- [ ] Mini mode works
- [ ] Micro mode works
- [ ] Settings persist after restart
- [ ] App cleans up on exit

### Test on Different Scenarios

If possible, test with:

- Ethernet adapter
- WiFi adapter
- Multiple adapters simultaneously
- VPN active
- Different Windows versions (10 vs 11)

---

## Debugging

### Frontend Debugging

1. **Open DevTools**
   - Press `F12` in the app
   - Or enable in `tauri.conf.json`: `"devtools": true`

2. **Check console for errors**
   - Look for TypeScript errors
   - Check network requests to Tauri backend

3. **Use Svelte DevTools**
   - Install Chrome extension
   - Inspect component state

### Backend Debugging

1. **Enable Rust logging**

   ```bash
   # Set environment variable before running
   set RUST_LOG=debug
   npm run tauri dev
   ```

2. **Check logs**
   - Logs printed to console
   - Also saved in: `%APPDATA%/dns-switcher/logs/`

3. **Use debug prints**

   ```rust
   use log::{debug, info, error};
   
   debug!("Debug message: {:?}", variable);
   info!("Info message");
   error!("Error: {}", error);
   ```

4. **Attach debugger**
   - VS Code launch configuration included
   - Press `F5` to start debugging

---

## Adding a New DNS Provider Preset

1. **Edit `src-tauri/src/presets.rs`**

2. **Add to the `presets` vector**

   ```rust
   DnsPreset {
       id: "provider_name".to_string(),
       name: "Provider Name".to_string(),
       icon: "🌐".to_string(),
       color: "#FF0000".to_string(),
       ipv4_primary: "1.2.3.4".to_string(),
       ipv4_secondary: Some("5.6.7.8".to_string()),
       ipv6_primary: Some("2001:db8::1".to_string()),
       ipv6_secondary: Some("2001:db8::2".to_string()),
       doh_template: Some("https://dns.example.com/dns-query".to_string()),
       description: Some("Description of the provider".to_string()),
       category: "privacy".to_string(), // or "speed", "security", "family"
       is_custom: false,
   },
   ```

3. **Test it**

   ```bash
   npm run tauri dev
   ```

4. **Submit PR** with provider info

---

## Code Review Process

1. **Automated checks** run on every PR:
   - TypeScript compilation
   - Prettier formatting
   - Rust compilation
   - Clippy lints

2. **Manual review** by maintainer:
   - Code quality
   - Follows project patterns
   - Properly tested
   - Documentation updated

3. **Feedback and iteration**
   - Address review comments
   - Push updates to same branch
   - PR automatically updates

4. **Merge**
   - Once approved, PR is merged
   - Your contribution is live!

---

## Release Process

(For maintainers)

1. **Update version** in:
   - `package.json`
   - `src-tauri/Cargo.toml`
   - `src-tauri/tauri.conf.json`

2. **Build release**

   ```bash
   npm run tauri build
   ```

3. **Test installer**
   - Install on clean Windows machine
   - Verify all features work

4. **Create Git tag**

   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

5. **GitHub Release**
   - Draft new release
   - Upload installer from `src-tauri/target/release/bundle/`
   - Write changelog
   - Publish

---

## Questions?

- **General questions:** Open a Discussion on GitHub
- **Bug reports:** Open an Issue with full details
- **Feature requests:** Open an Issue describing the feature
- **Code questions:** Comment on the relevant PR or file

---

## Code of Conduct

Be respectful, constructive, and professional. We're all here to make DNS Switcher better.

- Be kind to other contributors
- Accept constructive feedback
- Focus on what's best for the project
- Show empathy towards others

---

Thanks for contributing! 🎉

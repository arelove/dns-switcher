# Frequently Asked Questions

## General Questions

### What is DNS Switcher?

DNS Switcher is a desktop app for Windows that lets you change your DNS servers with one click. Instead of navigating through Windows network settings, you can switch between popular DNS providers instantly.

### Why would I need this?

Reasons to switch DNS:

- **Privacy:** Use DNS providers that don't log your queries (Cloudflare, Quad9)
- **Speed:** Some DNS servers are faster than your ISP's
- **Security:** Block malicious sites (Quad9, AdGuard)
- **Parental controls:** Filter inappropriate content (AdGuard Family, OpenDNS Family)
- **Development:** Test DNS configurations quickly
- **Bypass restrictions:** Access content blocked by your ISP's DNS

### Is this free?

Yes, completely free and open source. No ads, no premium version, no catches.

### Do I need to keep the app running?

No. Once you've set your DNS, you can close the app. The DNS settings persist until you change them again. The system tray icon is optional.

---

## Installation & Setup

### What are the system requirements?

- Windows 10 (build 17763 or later) or Windows 11
- ~5MB disk space for the app
- Administrator privileges (required for changing DNS)

### Why does it need administrator rights?

Changing DNS settings is a system-level operation that requires admin privileges. This is a Windows security feature, not a limitation of the app.

The app uses Windows UAC (User Account Control) to request elevation, so you'll see a prompt when you launch it.

### Can I install this on multiple computers?

Yes! Install on as many Windows PCs as you want. There's no license restriction.

### Will this work on Windows Server?

Probably, but it's untested. The app is designed for Windows 10/11 desktop.

---

## Usage

### How do I change DNS?

1. Launch DNS Switcher (run as administrator)
2. Select your network adapter from the dropdown
3. Click a DNS preset (Cloudflare, Google, etc.)
4. Wait a few seconds for the change to apply
5. Done!

### What's the difference between mini and micro mode?

- **Normal mode:** Full interface with all features
- **Mini mode:** Compact window with quick presets only
- **Micro mode:** Ultra-compact always-on-top widget

Use mini/micro mode when you want the app to stay accessible but take up minimal screen space.

### Can I add my own DNS servers?

Yes! Go to the "Services" tab and click "Add Custom List". You can create custom presets with your own DNS servers.

### How do I reset to default (automatic) DNS?

There's no "reset" button currently, but you can:

1. Close DNS Switcher
2. Go to Windows Settings → Network & Internet → Change adapter options
3. Right-click your adapter → Properties → IPv4 → Properties
4. Select "Obtain DNS server address automatically"

Or use this command (as administrator):

```cmd
netsh interface ipv4 set dnsservers "YOUR_ADAPTER_NAME" dhcp
```

### Does this change DNS for all programs?

Yes, it changes the system-wide DNS settings. All applications on your computer will use the new DNS servers.

---

## DNS Providers

### Which DNS provider should I use?

Depends on your priorities:

**For speed:** Cloudflare (1.1.1.1) or Google (8.8.8.8)
**For privacy:** Cloudflare, Quad9
**For security:** Quad9 (blocks malicious sites), AdGuard DNS
**For parental controls:** AdGuard Family, OpenDNS Family
**For general use:** Cloudflare or Google

Most people are happy with Cloudflare.

### What is DNS over HTTPS (DoH)?

DoH encrypts your DNS requests so your ISP can't see which websites you're visiting. Regular DNS is unencrypted and visible to your ISP.

**Note:** DoH only works on Windows 11 (build 22000+). Windows 10 does not support DoH configuration.

### Are these DNS providers safe?

Yes, all pre-configured providers are reputable:

- **Cloudflare:** Privacy-focused, no logging
- **Google:** Fast, reliable, collects some data
- **Quad9:** Security-focused, blocks malicious domains
- **OpenDNS:** Content filtering, parental controls
- **AdGuard:** Blocks ads and trackers

Always research a DNS provider before using custom servers from unknown sources.

### Can DNS providers see my traffic?

DNS providers can see:

- ✅ Which domain names you query (example.com)
- ✅ When you make the query

DNS providers cannot see:

- ❌ Full URLs (they see google.com but not google.com/search?q=...)
- ❌ Page content
- ❌ Login credentials
- ❌ HTTPS traffic content

If you want full privacy, use a VPN. DNS alone is not sufficient.

---

## Technical Questions

### Does this work with VPNs?

Yes and no. The app works, but most VPNs override DNS settings for security reasons.

**What happens:**

1. You set DNS to Cloudflare
2. You connect to VPN
3. VPN changes DNS to its own servers
4. Your traffic uses VPN's DNS (this is expected)

**To use custom DNS with a VPN:**

- Some VPNs allow custom DNS in settings
- Or set DNS after connecting to VPN (may reduce privacy)
- Or use split tunneling (if supported)

For maximum privacy, let your VPN handle DNS.

### Will this slow down my internet?

No. DNS only affects the initial connection to a website. Once your computer knows the IP address, DNS isn't involved anymore.

A slow DNS server might add 10-100ms to the initial connection, but you won't notice this in normal browsing.

### Does this work with IPv6?

Yes! The app supports both IPv4 and IPv6 DNS servers. If your ISP supports IPv6, you can configure both.

### What happens to my DNS when I restart Windows?

Your DNS settings persist across reboots. They don't reset unless you change them manually or switch to automatic (DHCP).

### Can I set different DNS for different network adapters?

Currently, no. The app sets the same DNS on all active adapters. This is intentional to avoid confusion.

Future versions might support per-adapter DNS.

### Can I schedule DNS changes?

Not built-in, but you could:

1. Use Windows Task Scheduler
2. Call `netsh` commands with your desired DNS
3. Schedule the task to run at specific times

This is advanced and not officially supported.

---

## Troubleshooting

### The app won't launch

- Run as administrator (right-click → Run as administrator)
- Install [Visual C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe)
- Check Windows version (must be 10 build 17763+ or Windows 11)
- Check if antivirus is blocking it

### DNS changes don't take effect

- Make sure you ran as administrator
- Flush DNS cache: `ipconfig /flushdns`
- Restart your browser
- Wait 1-2 minutes for changes to propagate
- Check [full troubleshooting guide](./TROUBLESHOOTING.md)

### "Access Denied" errors

- Always run the app as administrator
- Close VPN clients before changing DNS
- Make sure network adapter is enabled
- Try renaming adapter to simple name (no special characters)

### Changes revert after a while

- Check if you have a VPN running
- Some network management software may override DNS
- Windows might reset to DHCP if adapter is disabled/re-enabled

See the [Troubleshooting Guide](./TROUBLESHOOTING.md) for detailed solutions.

---

## Privacy & Security

### Does the app collect my data?

**No.** The app:

- Does not connect to the internet (except DNS providers you choose)
- Does not send telemetry
- Does not track usage
- Does not store browsing history
- All data stays on your computer

It's open source - you can verify this by reading the code.

### Where is my data stored?

Custom presets and settings are stored locally:

```text
C:\Users\YOUR_USERNAME\AppData\Roaming\dns-switcher\
```

This includes:

- Custom DNS presets
- App settings
- Window positions
- Logs

No data is sent anywhere.

### Is the app code audited?

The code is open source on GitHub. Anyone can review it. For critical security, you can:

1. Review the source code yourself
2. Build from source
3. Have a security expert audit it

No formal security audit has been performed yet.

### Can malware use this app to redirect my traffic?

If malware has admin rights on your computer, it can change DNS with or without this app. This app doesn't add new security risks.

To stay safe:

- Don't run unknown executables as admin
- Keep Windows Defender enabled
- Download the app only from official sources

---

## Comparison with Alternatives

### How is this different from manually changing DNS in Windows?

**DNS Switcher:**

- One click to change DNS
- Pre-configured popular providers
- Can save custom presets
- System tray for quick access
- Faster than navigating Windows settings

**Manual Windows method:**

- Free (built-in)
- No extra software needed
- More control over advanced settings
- Takes 5-10 clicks every time

### How is this different from DNS Jumper or similar tools?

Most DNS switching tools are similar. DNS Switcher is:

- Modern UI (not from 2005)
- Built with Tauri (smaller, faster than Electron)
- Open source
- Actively maintained
- Native Windows integration

### Should I use this instead of changing DNS in my router?

**Router DNS:**

- Affects all devices on your network
- Set once, forget it
- Can't easily switch between providers
- Requires router access

**DNS Switcher:**

- Affects only your computer
- Easy to switch providers
- Good for testing or specific needs
- Works even if you don't control the router

Both have their place. Use router DNS for household-wide changes, DNS Switcher for your PC.

---

## Development & Contributing

### Is this open source?

Yes! Apache 2.0 license. You can:

- View the source code
- Modify it
- Distribute it
- Use it commercially
- Contribute improvements

### How can I contribute?

See the [Contributing Guide](./CONTRIBUTING.md) for details on:

- Setting up development environment
- Code style
- How to submit changes
- Feature requests

### Can I request a feature?

Yes! Open an issue on GitHub describing:

- What you want
- Why it would be useful
- How it should work

No guarantees it'll be implemented, but all ideas are considered.

### Will there be a macOS or Linux version?

Maybe! It's a lot of work because:

- Different DNS management commands
- Different UI frameworks
- Different packaging

If you want to help port it, open an issue to discuss.

---

## Miscellaneous

### What languages is this written in?

- Frontend: TypeScript + Svelte
- Backend: Rust
- Framework: Tauri

### Why Tauri instead of Electron?

- **Smaller:** 5MB vs 150MB installers
- **Faster:** Native Rust backend
- **Less RAM:** ~20MB vs ~200MB
- **Better security:** Rust memory safety

### Can I use this commercially?

Yes, Apache 2.0 license allows commercial use. Just keep the license file.

### How do I uninstall?

1. Close the app completely (check system tray)
2. Go to Settings → Apps → DNS Switcher → Uninstall
3. (Optional) Delete leftover data in `%APPDATA%\dns-switcher\`

### Will this interfere with my antivirus?

Some antivirus software may flag it because it requires admin rights. This is a false positive. You can:

- Add to antivirus whitelist
- Download only from official GitHub releases
- Build from source if concerned

---

## Still have questions?

- Check the [Troubleshooting Guide](./TROUBLESHOOTING.md)
- Search [existing issues](../../issues) on GitHub
- Open a new issue if you can't find an answer
- Start a [discussion](../../discussions) for general questions

**Response time:** Usually 24-48 hours for GitHub issues.

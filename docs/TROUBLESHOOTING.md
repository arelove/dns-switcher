# Troubleshooting Guide

Having issues with DNS Switcher? This guide covers common problems and their solutions.

## Quick Diagnostics

**Before reporting a bug**, try these steps:

1. Run the app as Administrator (right-click → Run as administrator)
2. Restart the application
3. Check Windows Event Viewer for errors
4. Verify your network adapter is enabled

---

## Common Issues

### App Won't Launch

**Symptom:** Double-clicking the app does nothing, or it crashes immediately.

**Solutions:**

1. **Run as Administrator**
   - Right-click DNS Switcher
   - Select "Run as administrator"
   - The app MUST have admin rights to change DNS settings

2. **Install Visual C++ Redistributable**
   - Download from [Microsoft's website](https://aka.ms/vs/17/release/vc_redist.x64.exe)
   - Install and restart your computer

3. **Check Windows Version**
   - DNS Switcher requires Windows 10 (build 17763+) or Windows 11
   - Press `Win + R`, type `winver`, and check your version
   - If you're on an older build, update Windows

4. **Antivirus Blocking**
   - Some antivirus software flags the app due to admin requirements
   - Add DNS Switcher to your antivirus whitelist
   - Temporarily disable antivirus to test (not recommended long-term)

---

### "Access Denied" or Permission Errors

**Symptom:** Error messages about permissions when trying to change DNS.

**Solutions:**

1. **Always run as administrator**
   - DNS changes require elevated privileges
   - Right-click → Run as administrator

2. **UAC is blocking the app**
   - Go to Control Panel → User Accounts → Change User Account Control settings
   - Don't disable UAC entirely, but make sure it's not set to maximum
   - The app will request elevation through UAC prompts

3. **Network adapter locked by another program**
   - Close VPN clients (NordVPN, ExpressVPN, etc.)
   - Close other network management tools
   - Restart Windows networking service:

     ```cmd
     net stop "Network Connections"
     net start "Network Connections"
     ```

---

### DNS Not Changing / "Failed to set DNS"

**Symptom:** You select a DNS preset but nothing happens, or you get error messages.

**Solutions:**

1. **Check network adapter name**
   - Open Settings → Network & Internet → Change adapter options
   - Note the EXACT name of your adapter
   - Names with special characters (é, ñ, etc.) can cause issues
   - Rename adapter to simple English name: "Ethernet" or "WiFi"

2. **Adapter is disabled**
   - Make sure your network adapter is enabled
   - Settings → Network & Internet → Status
   - Click "Change adapter options" and verify it's not disabled (grayed out)

3. **Multiple network adapters**
   - DNS Switcher changes DNS on ALL active adapters
   - If you have Ethernet + WiFi, both will be affected
   - This is by design to ensure consistent DNS

4. **Restart network adapter**

   ```cmd
   # Run as administrator
   netsh interface set interface "YOUR_ADAPTER_NAME" disable
   netsh interface set interface "YOUR_ADAPTER_NAME" enable
   ```

5. **Flush DNS cache**
   - The app should do this automatically, but you can manually flush:

   ```cmd
   ipconfig /flushdns
   ```

---

### VPN Overriding DNS Settings

**Symptom:** DNS changes work but revert when VPN connects.

**Explanation:** Most VPN software overrides DNS to prevent leaks. This is normal behavior.

**Solutions:**

1. **Set DNS before connecting to VPN**
   - Change DNS settings first
   - Then connect to your VPN
   - VPN will use its own DNS (this is expected)

2. **Configure VPN to use custom DNS**
   - Some VPNs allow custom DNS servers
   - Check your VPN settings for "DNS" or "Custom DNS" option
   - Set it to your preferred DNS provider

3. **Split tunneling**
   - Some VPNs support split tunneling
   - Configure VPN to not route DNS traffic
   - (Not recommended for privacy)

**Note:** If privacy is your goal, let the VPN use its own DNS servers.

---

### DoH (DNS over HTTPS) Not Working

**Symptom:** Can't enable DoH, or setting shows as disabled.

**Solutions:**

1. **Check Windows Version**
   - DoH requires Windows 11 (build 22000+)
   - Windows 10 does NOT support DoH configuration via netsh
   - Press `Win + R`, type `winver` to check version

2. **Windows 11 only feature**
   - If you're on Windows 10, DoH settings won't work
   - You can still use regular DNS servers
   - Consider upgrading to Windows 11 for DoH support

3. **Verify DoH template**
   - Not all DNS providers support DoH
   - Check the DNS provider's documentation
   - Cloudflare, Google, Quad9, and AdGuard all support DoH

---

### App Crashes or Freezes

**Symptom:** App becomes unresponsive or crashes during use.

**Solutions:**

1. **Check logs**
   - Logs are stored in: `C:\Users\YOUR_USERNAME\AppData\Roaming\dns-switcher\logs\`
   - Look for error messages
   - Share logs when reporting bugs

2. **Corrupted settings**
   - Close the app
   - Delete config file: `C:\Users\YOUR_USERNAME\AppData\Roaming\dns-switcher\config\`
   - Restart the app (settings will reset)

3. **System resources**
   - DNS Switcher uses ~20MB RAM normally
   - If it's using much more, something's wrong
   - Restart the app

4. **Reinstall**
   - Uninstall completely
   - Delete leftover files in `AppData\Roaming\dns-switcher\`
   - Download fresh installer
   - Install again

---

### Custom Presets Not Saving

**Symptom:** Added custom DNS presets disappear after restart.

**Solutions:**

1. **Check write permissions**
   - Config folder: `C:\Users\YOUR_USERNAME\AppData\Roaming\dns-switcher\`
   - Make sure you have write permissions
   - Try running as administrator

2. **File corruption**
   - Delete `custom_presets.json` in config folder
   - Restart app
   - Re-add your custom presets

3. **Invalid DNS format**
   - Make sure DNS addresses are valid IPv4 or IPv6
   - IPv4 format: `8.8.8.8`
   - IPv6 format: `2001:4860:4860::8888`
   - No spaces, letters, or special characters

---

### Tray Icon Not Appearing

**Symptom:** System tray icon is missing or doesn't show.

**Solutions:**

1. **Tray overflow**
   - Click the up arrow (^) in system tray
   - DNS Switcher icon might be hidden there
   - Drag it to visible area

2. **Windows tray settings**
   - Right-click taskbar → Taskbar settings
   - Scroll to "Other system tray icons"
   - Make sure DNS Switcher is set to "On"

3. **Restart explorer.exe**

   ```cmd
   taskkill /f /im explorer.exe
   start explorer.exe
   ```

---

### Mini/Micro Mode Issues

**Symptom:** Mini or micro window doesn't appear or misbehaves.

**Solutions:**

1. **Window off-screen**
   - The window might be positioned outside visible area
   - Delete config file to reset window positions
   - `C:\Users\YOUR_USERNAME\AppData\Roaming\dns-switcher\config\window_positions.json`

2. **Always-on-top not working**
   - This is a known Windows limitation with some apps
   - Try toggling the setting off and on again
   - Restart the app

3. **Can't close mini window**
   - Use the tray icon to toggle mini mode off
   - Or close the main app entirely

---

### High CPU or Memory Usage

**Symptom:** DNS Switcher using too many resources.

**Normal usage:**

- RAM: ~200MB
- CPU: <1% when idle
- Disk: Minimal

**If much higher:**

1. **Background DNS testing**
   - App tests DNS latency periodically
   - This is normal and brief
   - If constant, something's wrong - restart app

2. **Multiple instances running**
   - Check Task Manager
   - Kill duplicate instances
   - Only one should be running

3. **Report as bug**
   - If consistently high usage
   - Note what actions trigger it
   - Share logs when reporting

---

## Network-Specific Issues

### DNS Changes Don't Take Effect Immediately

**This is normal.** DNS changes can take up to 5 minutes to fully propagate.

**Speed it up:**

1. Flush DNS cache:

   ```cmd
   ipconfig /flushdns
   ```

2. Release and renew IP:

   ```cmd
   ipconfig /release
   ipconfig /renew
   ```

3. Restart browser or app you're using

4. In worst case, restart computer

### Some Websites Still Use Old DNS

**Symptom:** Changed DNS but some sites resolve differently.

**Causes:**

1. **Browser DNS cache**
   - Chrome: Type `chrome://net-internals/#dns` and click "Clear host cache"
   - Firefox: Close and reopen browser
   - Edge: Clear browsing data

2. **Application-level DNS**
   - Some apps (Discord, Spotify) cache DNS
   - Restart the application

3. **ISP DNS hijacking**
   - Some ISPs intercept DNS requests
   - Use DoH (Windows 11 only) to bypass this
   - Or use a VPN

### Can't Access Certain Websites After DNS Change

**Symptom:** Some sites work, others don't.

**Solutions:**

1. **DNS provider blocking**
   - Some DNS providers (AdGuard, Quad9) block malicious sites
   - This is a feature, not a bug
   - Switch to Google or Cloudflare if you need unrestricted access

2. **Regional restrictions**
   - Some DNS providers use different servers by region
   - Try a different DNS provider

3. **Firewall blocking**
   - Check Windows Firewall
   - Make sure DNS ports (53, 853, 443) aren't blocked

4. **Flush and reset**

   ```cmd
   ipconfig /flushdns
   netsh winsock reset
   netsh int ip reset
   # Restart computer
   ```

---

## Still Having Issues?

### Gather Debug Information

Before reporting a bug, collect this info:

1. **Windows Version**

   ```cmd
   winver
   ```

   Take a screenshot

2. **Network Adapter Info**

   ```cmd
   ipconfig /all
   ```

   Copy output

3. **Current DNS**

   ```cmd
   nslookup google.com
   ```

   Note what server responds

4. **App Logs**
   - Location: `C:\Users\YOUR_USERNAME\AppData\Roaming\dns-switcher\logs\`
   - Include the most recent log file

5. **Error Message**
   - Take a screenshot
   - Copy exact error text

### Report a Bug

Open an issue on GitHub with:

- Description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Debug information from above
- Screenshots if applicable

**GitHub Issues:** [https://github.com/YOUR_USERNAME/dns-switcher/issues](../../issues)

---

## Advanced Troubleshooting

### Manual DNS Configuration

If the app isn't working, you can set DNS manually:

1. Open Command Prompt **as Administrator**

2. List network adapters:

   ```cmd
   netsh interface show interface
   ```

3. Set DNS (replace `YOUR_ADAPTER` and `DNS_IP`):

   ```cmd
   netsh interface ipv4 set dnsservers "YOUR_ADAPTER" static DNS_IP primary
   netsh interface ipv4 add dnsservers "YOUR_ADAPTER" SECONDARY_DNS index=2
   ```

    Example for Cloudflare on WiFi:

    ```cmd
    netsh interface ipv4 set dnsservers "Wi-Fi" static 1.1.1.1 primary
    netsh interface ipv4 add dnsservers "Wi-Fi" 1.0.0.1 index=2
    ```

4. Flush DNS:

   ```cmd
   ipconfig /flushdns
   ```

### Reset to DHCP (Automatic)

To revert to automatic DNS:

```cmd
netsh interface ipv4 set dnsservers "YOUR_ADAPTER" dhcp
```

### Complete Network Reset

Nuclear option - resets all network settings:

```cmd
netsh winsock reset
netsh int ip reset
ipconfig /release
ipconfig /renew
ipconfig /flushdns
```

**Then restart your computer.**

---

## FAQ

**Q: Do I need to keep the app running?**  
A: No. Once DNS is set, you can close the app. Settings persist until you change them.

**Q: Will this slow down my internet?**  
A: No. The app only changes which DNS server your computer asks. After setup, there's no performance impact.

**Q: Is this safe?**  
A: Yes. The app only changes DNS settings using standard Windows commands. No data is collected or sent anywhere.

**Q: Can I use this on multiple computers?**  
A: Yes. Install on as many Windows PCs as you want.

**Q: Does this work with VPNs?**  
A: Yes, but VPNs usually override DNS for security. Set DNS before connecting to VPN, or configure your VPN to use custom DNS.

**Q: Will this work on Windows Server?**  
A: Should work, but untested. Designed for Windows 10/11 desktop.

**Q: Can I set different DNS per network?**  
A: Not currently. All active adapters get the same DNS. This is a planned feature.

**Q: Why does UAC prompt appear?**  
A: Changing DNS requires administrator privileges. This is a Windows security feature.

---

## Getting Help

1. **Check this guide first** - most issues are covered here
2. **Search existing issues** on GitHub - someone might have had the same problem
3. **Ask in Discussions** for general questions
4. **Open an issue** for bugs with full debug info

**Response time:** Usually within 24-48 hours.

---

**Still stuck?** Open an issue with detailed information and I'll help you out.

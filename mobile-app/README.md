# XFTradesmen Mobile App

Tauri wrapper that loads the deployed XFTradesmen website in a native mobile webview.

## How It Works

This is a **thin wrapper** - it simply opens `https://xftradesmen.fly.dev` in a native webview.
No code from the main Leptos frontend is modified.

## Prerequisites

### iOS (requires macOS)
```bash
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
# Install Xcode 16+ from App Store
```

### Android
```bash
rustup target add aarch64-linux-android armv7-linux-androideabi
# Install Android Studio with SDK 24+
```

## Development

```bash
cd mobile-app/src-tauri

# iOS Simulator
cargo tauri ios dev

# Android Emulator
cargo tauri android dev
```

## Production Build

```bash
# iOS (.ipa for App Store)
cargo tauri ios build

# Android (.apk + .aab for Play Store)
cargo tauri android build
```

## Configuration

Edit `src-tauri/tauri.conf.json` to change:
- `devUrl` / `frontendDist` - Target URL
- `productName` - App name
- `identifier` - Bundle ID (com.xftradesmen.app)

## Icons

Add app icons to `src-tauri/icons/`:
- `32x32.png`
- `128x128.png`
- `128x128@2x.png`
- `icon.icns` (macOS)
- `icon.ico` (Windows)

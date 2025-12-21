@echo off
setlocal EnableDelayedExpansion

:: Title: XFTradesmen Complete Reset Script
:: This script performs a full cleanup of all build artifacts and caches

echo ========================================================
echo   XFTRADESMEN: COMPLETE ENVIRONMENT RESET
echo ========================================================
echo.
echo WARNING: This will delete all build artifacts and caches!
echo Press Ctrl+C to cancel, or
pause

:: Stop all running processes
echo [1/7] Stopping all development processes...
taskkill /F /FI "WINDOWTITLE eq XFTradesmen*" 2>nul
taskkill /F /FI "WINDOWTITLE eq Tailwind*" 2>nul
echo Done.

:: Clean Rust/Cargo artifacts
echo [2/7] Cleaning Rust build artifacts...
cargo clean
echo Done.

:: Clean Node modules and lock files
echo [3/7] Cleaning Node.js artifacts...
if exist "node_modules" (
    echo Removing node_modules...
    rmdir /s /q node_modules
)
if exist "package-lock.json" (
    echo Removing package-lock.json...
    del /f /q package-lock.json
)
echo Done.

:: Clean Leptos/WASM artifacts
echo [4/7] Cleaning Leptos artifacts...
if exist "target\site" (
    rmdir /s /q target\site
)
if exist "target\front" (
    rmdir /s /q target\front
)
echo Done.

:: Clean CSS build output
echo [5/7] Cleaning CSS artifacts...
if exist "frontend-leptos\public\xftradesmen.css" (
    del /f /q frontend-leptos\public\xftradesmen.css
)
echo Done.

:: Clean cargo registry cache (optional - uncomment if needed)
:: echo [6/7] Cleaning cargo registry cache...
:: cargo clean --release
:: echo Done.

:: Reinstall dependencies
echo [6/7] Reinstalling npm dependencies...
call npm install
if errorlevel 1 (
    echo [ERROR] Failed to install npm dependencies
    pause
    exit /b 1
)
echo Done.

:: Build CSS
echo [7/7] Building CSS...
call npm run build:css
if errorlevel 1 (
    echo [ERROR] Failed to build CSS
    pause
    exit /b 1
)
echo Done.

echo.
echo ========================================================
echo   RESET COMPLETE!
echo   Run start.bat to launch the development environment
echo ========================================================
echo.
pause

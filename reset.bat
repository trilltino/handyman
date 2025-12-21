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
echo [1/8] Stopping all development processes...
taskkill /F /FI "WINDOWTITLE eq XFTradesmen*" 2>nul
taskkill /F /FI "WINDOWTITLE eq Tailwind*" 2>nul
taskkill /F /IM cargo.exe /T 2>nul
taskkill /F /IM cargo-leptos.exe /T 2>nul
taskkill /F /IM rust-analyzer.exe /T 2>nul
taskkill /F /IM node.exe /T 2>nul
echo Done.

:: Wait for processes to fully terminate
echo Waiting for processes to release locks...
timeout /t 3 /nobreak >nul

:: Clean Rust/Cargo artifacts
echo [2/8] Cleaning Rust build artifacts...
cargo clean 2>nul
if errorlevel 1 (
    echo Warning: Some files couldn't be deleted (may be locked)
    echo Attempting manual cleanup...
    rmdir /s /q target 2>nul
)
echo Done.

:: Clean Node modules and lock files
echo [3/8] Cleaning Node.js artifacts...
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
echo [4/8] Cleaning Leptos artifacts...
if exist "target\site" (
    rmdir /s /q target\site 2>nul
)
if exist "target\front" (
    rmdir /s /q target\front 2>nul
)
echo Done.

:: Clean CSS build output
echo [5/8] Cleaning CSS artifacts...
if exist "frontend-leptos\public\xftradesmen.css" (
    del /f /q frontend-leptos\public\xftradesmen.css
)
echo Done.

:: Clean any remaining build artifacts
echo [6/8] Cleaning additional artifacts...
rmdir /s /q target\debug 2>nul
rmdir /s /q target\release 2>nul
echo Done.

:: Reinstall dependencies
echo [7/8] Reinstalling npm dependencies...
call npm install
if errorlevel 1 (
    echo [ERROR] Failed to install npm dependencies
    pause
    exit /b 1
)
echo Done.

:: Build CSS
echo [8/8] Building CSS...
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

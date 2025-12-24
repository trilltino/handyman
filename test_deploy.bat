@echo off
REM Fly.io Deployment Test Script for Windows
REM Run this locally before pushing to verify build will succeed

echo ========================================
echo   XFHandyman Deployment Test
echo ========================================
echo.

REM 1. Check Rust toolchain
echo [INFO] Checking Rust toolchain...
rustup show | findstr /C:"installed" >nul 2>&1
if %errorlevel% neq 0 (
    echo [FAIL] Rust not installed
    exit /b 1
)
rustup target list --installed | findstr /C:"wasm32-unknown-unknown" >nul 2>&1
if %errorlevel% neq 0 (
    echo [FAIL] wasm32 target not installed. Run: rustup target add wasm32-unknown-unknown
    exit /b 1
)
echo [PASS] Rust toolchain OK

REM 2. Check cargo-leptos
echo [INFO] Checking cargo-leptos...
cargo leptos --version >nul 2>&1
if %errorlevel% neq 0 (
    echo [FAIL] cargo-leptos not installed
    exit /b 1
)
echo [PASS] cargo-leptos OK

REM 3. Check Node.js
echo [INFO] Checking Node.js...
node --version >nul 2>&1
if %errorlevel% neq 0 (
    echo [FAIL] Node.js not installed
    exit /b 1
)
echo [PASS] Node.js OK

REM 4. Check formatting
echo [INFO] Checking code formatting...
cargo fmt --all -- --check
if %errorlevel% neq 0 (
    echo [FAIL] Code not formatted. Run: cargo fmt --all
    exit /b 1
)
echo [PASS] Formatting OK

REM 5. Build CSS
echo [INFO] Building CSS...
call npm run build:css
if %errorlevel% neq 0 (
    echo [FAIL] CSS build failed
    exit /b 1
)
echo [PASS] CSS build OK

REM 6. Check SSR build
echo [INFO] Checking SSR build...
cargo check -p frontend-leptos --features ssr
if %errorlevel% neq 0 (
    echo [FAIL] SSR check failed
    exit /b 1
)
echo [PASS] SSR check OK

REM 7. Check API build
echo [INFO] Checking API build...
cargo check -p api
if %errorlevel% neq 0 (
    echo [FAIL] API check failed
    exit /b 1
)
echo [PASS] API check OK

REM 8. Full build (optional)
if "%1"=="--full" (
    echo [INFO] Running full Leptos build ^(this takes ~5-10 min^)...
    cargo leptos build --release
    if %errorlevel% neq 0 (
        echo [FAIL] Leptos build failed
        exit /b 1
    )
    echo [PASS] Full Leptos build OK
)

echo.
echo ========================================
echo   All checks passed!
echo ========================================
echo.
echo Ready to deploy. Run:
echo   git add -A ^&^& git commit -m "your message" ^&^& git push origin main
echo.
echo Or run the full build test with:
echo   test_deploy.bat --full

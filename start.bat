@echo off
setlocal EnableDelayedExpansion

:: Title: XFTradesmen Local Development Launcher
:: Uses npm for Tailwind CSS v4

echo ========================================================
echo   XFTRADESMEN: LOCAL DEVELOPMENT STARTUP
echo ========================================================
echo.

:: Check if .env file exists
if not exist ".env" (
    echo [WARNING] .env file not found!
    echo Please copy .env.example to .env and configure your settings.
    echo.
    pause
    exit /b 1
)

:: Ensure npm dependencies are installed
if not exist "node_modules" (
    echo Installing npm dependencies...
    call npm install
    if errorlevel 1 (
        echo [ERROR] Failed to install npm dependencies
        pause
        exit /b 1
    )
)

:: Build CSS first (before starting servers)
echo [1/4] Building Tailwind CSS...
call npm run build:css
if errorlevel 1 (
    echo [ERROR] Failed to build CSS
    pause
    exit /b 1
)

:: Start CSS watch in background (rebuilds on file changes)
echo [2/4] Starting CSS watch...
start "Tailwind Watch" cmd /k "npm run watch:css"
timeout /t 1 /nobreak >nul

:: Start Backend API (dev mode for faster builds)
:: Set explicit port and logging
set API_PORT=8080
set RUST_LOG=info
echo [3/4] Launching Backend API (dev mode on port %API_PORT%)...
start "XFTradesmen Backend" cmd /k "set API_PORT=8080 && set RUST_LOG=info && cargo run -p api"
timeout /t 2 /nobreak >nul

:: Start Frontend with cargo leptos (dev mode)
:: Set API_URL to point to local backend
set API_URL=http://127.0.0.1:%API_PORT%
echo [4/4] Launching Frontend (dev mode proxying to %API_URL%)...
timeout /t 1 /nobreak >nul
start "XFTradesmen Frontend" cmd /k "set API_URL=http://127.0.0.1:8080 && cargo leptos watch"

echo.
echo ========================================================
echo   SYSTEMS LAUNCHED (Development Mode with Live Reload)
echo   Frontend: http://127.0.0.1:3001
echo   Backend:  http://127.0.0.1:8080
echo   Reload Port: 3002
echo ========================================================
echo.
echo LIVE RELOAD ACTIVE:
echo   - Rust code changes auto-reload
echo   - CSS changes auto-rebuild and refresh
echo   - Browser auto-refreshes on changes
echo.
echo Press any key to exit (servers will continue running)...
pause >nul

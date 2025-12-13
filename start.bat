@echo off
setlocal EnableDelayedExpansion

:: Title: Handyman Marketplace Optimized Launcher
:: Uses npm for Tailwind CSS v4

echo ========================================================
echo   HANDYMAN MARKETPLACE: OPTIMIZED STARTUP
echo ========================================================
echo.

:: Ensure npm dependencies are installed
if not exist "node_modules" (
    echo Installing npm dependencies...
    call npm install
)

:: 1. Build CSS with Tailwind
echo [1/4] Building Tailwind CSS...
call npm run build:css
if !errorlevel! neq 0 (
    echo ERROR: CSS build failed
    pause
    exit /b 1
)

:: 2. Start Backend API
echo [2/4] Launching Backend API...
start "Handyman Backend API" cmd /k "cargo run -p api --release"

:: 3. Start Frontend
echo [3/4] Launching Frontend...
timeout /t 2 /nobreak >nul
start "Handyman Frontend" cmd /k "cargo leptos serve --release"

:: 4. Start CSS watch
echo [4/4] Starting CSS watch...
timeout /t 2 /nobreak >nul
start "Tailwind Watch" cmd /k "npm run watch:css"

echo.
echo ========================================================
echo   SYSTEMS LAUNCHED
echo   Frontend: http://127.0.0.1:3001
echo   Backend:  http://127.0.0.1:8080
echo ========================================================
pause >nul

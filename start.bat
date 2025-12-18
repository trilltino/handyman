@echo off
setlocal EnableDelayedExpansion

:: Title: XFTradesmen Launcher
:: Uses npm for Tailwind CSS v4

echo ========================================================
echo   XFTRADESMEN: STARTUP
echo ========================================================
echo.

:: Ensure npm dependencies are installed
if not exist "node_modules" (
    echo Installing npm dependencies...
    call npm install
)

:: 1. Start Backend API
echo [1/4] Launching Backend API...
start "XFTradesmen Backend" cmd /k "cd backend && cargo run -p api --release"

:: 2. Start Frontend (cargo leptos)
echo [2/4] Launching Frontend...
timeout /t 2 /nobreak >nul
start "XFTradesmen Frontend" cmd /k "cargo leptos serve --release"

:: 3. Wait for cargo leptos to initialize, then build CSS
echo [3/4] Building Tailwind CSS...
timeout /t 5 /nobreak >nul
call npm run build:css

:: 4. Start CSS watch (rebuilds on file changes)
echo [4/4] Starting CSS watch...
start "Tailwind Watch" cmd /k "npm run watch:css"

echo.
echo ========================================================
echo   SYSTEMS LAUNCHED
echo   Frontend: http://127.0.0.1:3001
echo   Backend:  http://127.0.0.1:8080
echo ========================================================
echo.
echo CSS will be rebuilt automatically when you edit input.css
pause >nul


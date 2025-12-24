@echo off
REM GitHub Actions Log Downloader and Error Extractor
REM Requires: GitHub CLI (gh) - install with: winget install GitHub.cli
REM 
REM Usage:
REM   fetch_logs.bat              - Fetch latest failed run logs
REM   fetch_logs.bat 12345678     - Fetch specific run ID logs
REM   fetch_logs.bat --list       - List recent workflow runs

setlocal enabledelayedexpansion

set REPO=trilltino/handyman
set LOG_DIR=%~dp0logs
set ERRORS_FILE=%LOG_DIR%\errors.txt

REM Create logs directory
if not exist "%LOG_DIR%" mkdir "%LOG_DIR%"

REM Check if gh CLI is installed
gh --version >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] GitHub CLI ^(gh^) not installed.
    echo Install with: winget install GitHub.cli
    echo Or download from: https://cli.github.com/
    exit /b 1
)

REM Check if authenticated
gh auth status >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Not authenticated with GitHub CLI.
    echo Run: gh auth login
    exit /b 1
)

REM Handle --list flag
if "%1"=="--list" (
    echo [INFO] Recent workflow runs for %REPO%:
    echo.
    gh run list --repo %REPO% --limit 10
    exit /b 0
)

REM Get run ID
if "%1"=="" (
    echo [INFO] Finding latest failed run...
    for /f "tokens=1" %%i in ('gh run list --repo %REPO% --status failure --limit 1 --json databaseId --jq ".[0].databaseId"') do set RUN_ID=%%i
    if "!RUN_ID!"=="" (
        echo [INFO] No failed runs found. Fetching latest run instead...
        for /f "tokens=1" %%i in ('gh run list --repo %REPO% --limit 1 --json databaseId --jq ".[0].databaseId"') do set RUN_ID=%%i
    )
) else (
    set RUN_ID=%1
)

if "!RUN_ID!"=="" (
    echo [ERROR] Could not determine run ID
    exit /b 1
)

echo [INFO] Downloading logs for run ID: !RUN_ID!
set ZIP_FILE=%LOG_DIR%\logs_!RUN_ID!.zip
set EXTRACT_DIR=%LOG_DIR%\run_!RUN_ID!

REM Download logs
gh run download !RUN_ID! --repo %REPO% --dir "%EXTRACT_DIR%" 2>nul
if %errorlevel% neq 0 (
    echo [INFO] No artifacts, downloading raw logs...
    gh run view !RUN_ID! --repo %REPO% --log > "%LOG_DIR%\run_!RUN_ID!.log" 2>&1
    set LOG_FILE=%LOG_DIR%\run_!RUN_ID!.log
) else (
    set LOG_FILE=%EXTRACT_DIR%
)

echo [INFO] Logs saved to: %LOG_DIR%
echo.

REM Extract errors
echo [INFO] Extracting errors...
echo ======================================== > "%ERRORS_FILE%"
echo   Errors from Run !RUN_ID! >> "%ERRORS_FILE%"
echo   Extracted: %date% %time% >> "%ERRORS_FILE%"
echo ======================================== >> "%ERRORS_FILE%"
echo. >> "%ERRORS_FILE%"

REM Search for error patterns in logs
if exist "%LOG_DIR%\run_!RUN_ID!.log" (
    findstr /i /n "error: Error: FAIL failed exit code 1" "%LOG_DIR%\run_!RUN_ID!.log" >> "%ERRORS_FILE%" 2>nul
)

if exist "%EXTRACT_DIR%" (
    for /r "%EXTRACT_DIR%" %%f in (*.txt *.log) do (
        echo --- From: %%f --- >> "%ERRORS_FILE%"
        findstr /i /n "error: Error: FAIL failed exit code 1" "%%f" >> "%ERRORS_FILE%" 2>nul
    )
)

echo.
echo [DONE] Errors extracted to: %ERRORS_FILE%
echo.
echo ========================================
echo   Error Summary:
echo ========================================
type "%ERRORS_FILE%"

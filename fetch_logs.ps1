# PowerShell script to fetch GitHub Actions logs and extract errors
# Requires: GitHub CLI (gh) - install with: winget install GitHub.cli
#
# Usage:
#   .\fetch_logs.ps1              - Fetch latest failed run logs
#   .\fetch_logs.ps1 -RunId 12345 - Fetch specific run ID logs  
#   .\fetch_logs.ps1 -List        - List recent workflow runs

param(
    [string]$RunId = "",
    [switch]$List,
    [switch]$Latest,
    [string]$Repo = "trilltino/handyman"
)

$LogDir = Join-Path $PSScriptRoot "logs"
$ErrorsFile = Join-Path $LogDir "errors.txt"

# Create logs directory
if (-not (Test-Path $LogDir)) {
    New-Item -ItemType Directory -Path $LogDir -Force | Out-Null
}

# Check if gh CLI is installed
try {
    gh --version | Out-Null
} catch {
    Write-Host "[ERROR] GitHub CLI (gh) not installed." -ForegroundColor Red
    Write-Host "Install with: winget install GitHub.cli"
    Write-Host "Or download from: https://cli.github.com/"
    exit 1
}

# Check if authenticated
$authStatus = gh auth status 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] Not authenticated with GitHub CLI." -ForegroundColor Red
    Write-Host "Run: gh auth login"
    exit 1
}

# Handle -List flag
if ($List) {
    Write-Host "[INFO] Recent workflow runs for $Repo`:" -ForegroundColor Cyan
    Write-Host ""
    gh run list --repo $Repo --limit 15
    exit 0
}

# Get run ID if not specified
if ([string]::IsNullOrEmpty($RunId)) {
    Write-Host "[INFO] Finding latest failed run..." -ForegroundColor Yellow
    $RunId = gh run list --repo $Repo --status failure --limit 1 --json databaseId --jq ".[0].databaseId" 2>$null
    
    if ([string]::IsNullOrEmpty($RunId)) {
        Write-Host "[INFO] No failed runs found. Fetching latest run instead..." -ForegroundColor Yellow
        $RunId = gh run list --repo $Repo --limit 1 --json databaseId --jq ".[0].databaseId"
    }
}

if ([string]::IsNullOrEmpty($RunId)) {
    Write-Host "[ERROR] Could not determine run ID" -ForegroundColor Red
    exit 1
}

Write-Host "[INFO] Downloading logs for run ID: $RunId" -ForegroundColor Cyan
$LogFile = Join-Path $LogDir "run_$RunId.log"

# Download logs
gh run view $RunId --repo $Repo --log 2>&1 | Out-File -FilePath $LogFile -Encoding utf8

Write-Host "[INFO] Logs saved to: $LogFile" -ForegroundColor Green
Write-Host ""

# Extract errors
Write-Host "[INFO] Extracting errors..." -ForegroundColor Yellow

$Header = @"
========================================
  Errors from Run $RunId
  Extracted: $(Get-Date)
========================================

"@

$Header | Out-File -FilePath $ErrorsFile -Encoding utf8

# Search for error patterns
$ErrorPatterns = @(
    "error:",
    "Error:",
    "FAIL",
    "failed",
    "exit code 1",
    "panicked",
    "cannot find",
    "not found"
)

$LogContent = Get-Content $LogFile -ErrorAction SilentlyContinue
$ErrorLines = @()

foreach ($line in $LogContent) {
    foreach ($pattern in $ErrorPatterns) {
        if ($line -match $pattern) {
            $ErrorLines += $line
            break
        }
    }
}

# Remove duplicates and write to file
$ErrorLines | Select-Object -Unique | Add-Content -Path $ErrorsFile

Write-Host ""
Write-Host "[DONE] Errors extracted to: $ErrorsFile" -ForegroundColor Green
Write-Host ""
Write-Host "========================================"  -ForegroundColor Cyan
Write-Host "  Error Summary (last 50 lines):"  -ForegroundColor Cyan
Write-Host "========================================"  -ForegroundColor Cyan
Write-Host ""

# Display errors
$Errors = Get-Content $ErrorsFile | Select-Object -Last 50
foreach ($line in $Errors) {
    if ($line -match "error:|Error:|FAIL|failed|panicked") {
        Write-Host $line -ForegroundColor Red
    } else {
        Write-Host $line
    }
}

Write-Host ""
Write-Host "[TIP] Full log at: $LogFile" -ForegroundColor Yellow
Write-Host "[TIP] To view all runs: .\fetch_logs.ps1 -List" -ForegroundColor Yellow

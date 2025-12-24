# PowerShell script to fetch GitHub Actions logs and extract errors
# Requires: GitHub CLI (gh) - install with: winget install GitHub.cli
#
# Usage:
#   .\fetch_logs.ps1              - Fetch latest failed run logs
#   .\fetch_logs.ps1 -RunId 12345 - Fetch specific run ID logs  
#   .\fetch_logs.ps1 -List        - List recent workflow runs
#   .\fetch_logs.ps1 -Artifacts   - Download artifacts from latest failed run

param(
    [string]$RunId = "",
    [switch]$List,
    [switch]$Latest,
    [switch]$Artifacts,
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
}
catch {
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

Write-Host "[INFO] Run ID: $RunId" -ForegroundColor Cyan

# Download artifacts if requested or by default
$ArtifactDir = Join-Path $LogDir "artifacts_$RunId"
Write-Host "[INFO] Downloading artifacts..." -ForegroundColor Yellow
gh run download $RunId --repo $Repo --dir $ArtifactDir 2>$null

if (Test-Path $ArtifactDir) {
    Write-Host "[INFO] Artifacts saved to: $ArtifactDir" -ForegroundColor Green
    
    # Check for errors.txt in artifacts
    $artifactErrors = Get-ChildItem -Path $ArtifactDir -Recurse -Filter "errors.txt" -ErrorAction SilentlyContinue
    if ($artifactErrors) {
        Write-Host ""
        Write-Host "========================================" -ForegroundColor Cyan
        Write-Host "  Errors from Artifacts:" -ForegroundColor Cyan
        Write-Host "========================================" -ForegroundColor Cyan
        foreach ($errFile in $artifactErrors) {
            Write-Host "--- $($errFile.FullName) ---" -ForegroundColor Yellow
            Get-Content $errFile.FullName | ForEach-Object {
                if ($_ -match "error|failed|cannot") {
                    Write-Host $_ -ForegroundColor Red
                }
                else {
                    Write-Host $_
                }
            }
        }
    }
}
else {
    Write-Host "[INFO] No artifacts found. Downloading raw logs..." -ForegroundColor Yellow
}

# Also download raw logs
$LogFile = Join-Path $LogDir "run_$RunId.log"
Write-Host "[INFO] Downloading raw logs..." -ForegroundColor Yellow
gh run view $RunId --repo $Repo --log 2>&1 | Out-File -FilePath $LogFile -Encoding utf8

Write-Host "[INFO] Logs saved to: $LogFile" -ForegroundColor Green

# Extract errors from raw logs
Write-Host "[INFO] Extracting errors from raw logs..." -ForegroundColor Yellow

$Header = @"
========================================
  Errors from Run $RunId
  Extracted: $(Get-Date)
========================================

"@

$Header | Out-File -FilePath $ErrorsFile -Encoding utf8

$ErrorPatterns = @(
    "error\[",
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

$ErrorLines | Select-Object -Unique | Add-Content -Path $ErrorsFile

Write-Host ""
Write-Host "[DONE] Analysis complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Files saved:" -ForegroundColor Cyan
Write-Host "  - Raw logs: $LogFile"
Write-Host "  - Errors: $ErrorsFile"
if (Test-Path $ArtifactDir) {
    Write-Host "  - Artifacts: $ArtifactDir"
}
Write-Host ""
Write-Host "[TIP] To list runs: .\fetch_logs.ps1 -List" -ForegroundColor Yellow

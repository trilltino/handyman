# PowerShell script to delete old GitHub Actions workflow runs
# Requires: GitHub CLI (gh)
#
# Usage:
#   .\clear_runs.ps1              - Delete all workflow runs (keeps last 5)
#   .\clear_runs.ps1 -KeepLast 10 - Keep last 10 runs
#   .\clear_runs.ps1 -DryRun      - Show what would be deleted without deleting

param(
    [int]$KeepLast = 5,
    [switch]$DryRun,
    [string]$Repo = "trilltino/handyman"
)

Write-Host "========================================"
Write-Host "  Clear GitHub Actions Workflow Runs"
Write-Host "========================================"
Write-Host ""

# Check GitHub CLI
try {
    gh --version | Out-Null
}
catch {
    Write-Host "[ERROR] GitHub CLI (gh) not installed." -ForegroundColor Red
    exit 1
}

# Get all workflow runs
Write-Host "[INFO] Fetching workflow runs..." -ForegroundColor Yellow
$runs = gh run list --repo $Repo --limit 500 --json databaseId, conclusion, displayTitle, createdAt 2>$null | ConvertFrom-Json

if (-not $runs) {
    Write-Host "[ERROR] Failed to fetch runs. Make sure you're authenticated with 'gh auth login'" -ForegroundColor Red
    exit 1
}

$total = $runs.Count
Write-Host "[INFO] Found $total workflow runs" -ForegroundColor Cyan

if ($total -le $KeepLast) {
    Write-Host "[INFO] Only $total runs exist, keeping all (threshold: $KeepLast)" -ForegroundColor Green
    exit 0
}

# Skip the most recent runs
$toDelete = $runs | Select-Object -Skip $KeepLast
$deleteCount = $toDelete.Count

Write-Host "[INFO] Will delete $deleteCount runs (keeping last $KeepLast)" -ForegroundColor Yellow
Write-Host ""

if ($DryRun) {
    Write-Host "[DRY RUN] Would delete these runs:" -ForegroundColor Magenta
    $toDelete | Select-Object -First 20 | ForEach-Object {
        Write-Host "  - Run $($_.databaseId): $($_.displayTitle) ($($_.conclusion)) - $($_.createdAt)"
    }
    if ($deleteCount -gt 20) {
        Write-Host "  ... and $($deleteCount - 20) more"
    }
    exit 0
}

# Delete runs
Write-Host "[INFO] Starting deletion..." -ForegroundColor Yellow
$deleted = 0
$failed = 0

foreach ($run in $toDelete) {
    try {
        gh run delete $run.databaseId --repo $Repo 2>$null
        $deleted++
        if ($deleted % 10 -eq 0) {
            Write-Host "[PROGRESS] Deleted $deleted of $deleteCount runs..." -ForegroundColor Cyan
        }
    }
    catch {
        $failed++
    }
}

Write-Host ""
Write-Host "[DONE] Deleted $deleted runs" -ForegroundColor Green
if ($failed -gt 0) {
    Write-Host "[WARN] Failed to delete $failed runs" -ForegroundColor Yellow
}
Write-Host ""
Write-Host "Remaining runs: $KeepLast" -ForegroundColor Cyan

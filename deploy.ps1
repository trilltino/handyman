# XFTradesman Deploy Script with Google Sitemap Ping
# Usage: .\deploy.ps1

Write-Host "🚀 Deploying to Fly.io..." -ForegroundColor Cyan

# Deploy to Fly.io
fly deploy

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Deployment successful!" -ForegroundColor Green
    
    # Ping Google to re-crawl the sitemap
    Write-Host "📡 Pinging Google to index sitemap..." -ForegroundColor Yellow
    
    try {
        $response = Invoke-WebRequest -Uri "https://www.google.com/ping?sitemap=https://xftradesmen.fly.dev/sitemap.xml" -UseBasicParsing
        if ($response.StatusCode -eq 200) {
            Write-Host "✅ Google sitemap ping successful!" -ForegroundColor Green
        } else {
            Write-Host "⚠️ Google ping returned status: $($response.StatusCode)" -ForegroundColor Yellow
        }
    } catch {
        Write-Host "⚠️ Google ping failed: $_" -ForegroundColor Yellow
    }

    # Also ping Bing
    Write-Host "📡 Pinging Bing to index sitemap..." -ForegroundColor Yellow
    try {
        $response = Invoke-WebRequest -Uri "https://www.bing.com/ping?sitemap=https://xftradesmen.fly.dev/sitemap.xml" -UseBasicParsing
        if ($response.StatusCode -eq 200) {
            Write-Host "✅ Bing sitemap ping successful!" -ForegroundColor Green
        } else {
            Write-Host "⚠️ Bing ping returned status: $($response.StatusCode)" -ForegroundColor Yellow
        }
    } catch {
        Write-Host "⚠️ Bing ping failed: $_" -ForegroundColor Yellow
    }

    Write-Host ""
    Write-Host "🎉 Deployment complete! Search engines have been notified." -ForegroundColor Green
    Write-Host "📋 Next steps:" -ForegroundColor Cyan
    Write-Host "   1. Verify sitemap at: https://xftradesmen.fly.dev/sitemap.xml" -ForegroundColor White
    Write-Host "   2. Submit to Google Search Console: https://search.google.com/search-console" -ForegroundColor White
    Write-Host "   3. Submit to Bing Webmaster Tools: https://www.bing.com/webmasters" -ForegroundColor White
} else {
    Write-Host "❌ Deployment failed!" -ForegroundColor Red
    exit 1
}

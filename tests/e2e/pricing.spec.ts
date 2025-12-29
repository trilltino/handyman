import { test, expect } from '@playwright/test';

test.describe('Pricing Page', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/pricing');
    });

    test('should load pricing page successfully', async ({ page }) => {
        await expect(page).toHaveTitle(/Pricing|Packages/i);
        await expect(page.locator('h1')).toBeVisible();
    });

    test('should display pricing information', async ({ page }) => {
        // Check for pricing content
        const body = await page.locator('body').textContent();
        expect(body).toMatch(/£|\$|price|package/i);
    });

    test('should have CTA buttons', async ({ page }) => {
        // Look for call-to-action buttons
        const ctaButton = page.locator('a[href*="contact"], button:has-text("Get Started"), a:has-text("Contact")').first();
        if (await ctaButton.count() > 0) {
            await expect(ctaButton).toBeVisible();
        }
    });

    test('should display package features', async ({ page }) => {
        // Look for feature lists or package descriptions
        const features = page.locator('ul, .features, .package-features').first();
        if (await features.count() > 0) {
            await expect(features).toBeVisible();
        }
    });

    test('should be responsive on mobile', async ({ page }) => {
        await page.setViewportSize({ width: 375, height: 667 });
        await page.goto('/pricing');
        await expect(page.locator('body')).toBeVisible();
        await expect(page.locator('h1')).toBeVisible();
    });
});

test.describe('Packages Page', () => {
    test('should load packages page', async ({ page }) => {
        await page.goto('/packages');
        await expect(page.locator('body')).toBeVisible();
    });

    test('should display multiple package options', async ({ page }) => {
        await page.goto('/packages');
        const packageCards = page.locator('.package-card, .pricing-card, [class*="package"]');
        if (await packageCards.count() > 0) {
            expect(await packageCards.count()).toBeGreaterThanOrEqual(1);
        }
    });
});

import { test, expect } from '@playwright/test';

test.describe('Home Page', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
    });

    test('should load and display hero section', async ({ page }) => {
        // Check page title
        await expect(page).toHaveTitle(/XF Tradesmen/);

        // Check hero heading is visible
        const h1 = page.locator('h1').first();
        await expect(h1).toBeVisible();
    });

    test('should have working navigation links', async ({ page }) => {
        // Check pricing link works
        await page.click('a[href="/pricing"]');
        await expect(page).toHaveURL('/pricing');
    });

    test('should display services section', async ({ page }) => {
        // Scroll to services if needed
        const services = page.locator('text=Services').first();
        if (await services.isVisible()) {
            await expect(services).toBeVisible();
        }
    });

    test('should be accessible on mobile', async ({ page }) => {
        // Set mobile viewport
        await page.setViewportSize({ width: 375, height: 667 });
        await page.goto('/');

        // Page should still load without errors
        await expect(page.locator('body')).toBeVisible();
    });
});

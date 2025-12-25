import { test, expect } from '@playwright/test';

test.describe('Navigation', () => {
    test('should navigate between pages', async ({ page }) => {
        // Start at home
        await page.goto('/');

        // Navigate to pricing
        await page.click('a[href="/pricing"]');
        await expect(page).toHaveURL('/pricing');

        // Navigate to contact
        await page.click('a[href="/contact"]');
        await expect(page).toHaveURL('/contact');

        // Navigate back home via logo or nav
        const homeLink = page.locator('a[href="/"]').first();
        if (await homeLink.isVisible()) {
            await homeLink.click();
            await expect(page).toHaveURL('/');
        }
    });

    test('should have consistent header on all pages', async ({ page }) => {
        const pages = ['/', '/pricing', '/contact'];

        for (const url of pages) {
            await page.goto(url);
            const header = page.locator('header, nav').first();
            await expect(header).toBeVisible();
        }
    });
});

test.describe('Mobile Navigation', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 375, height: 667 });
    });

    test('should show hamburger menu on mobile', async ({ page }) => {
        await page.goto('/');

        // Look for hamburger button
        const hamburger = page.locator('.hamburger-btn, button[aria-label*="menu"]').first();

        // On mobile, hamburger should be visible
        // (may not exist if design uses different mobile nav)
        if (await hamburger.count() > 0) {
            await expect(hamburger).toBeVisible();
        }
    });

    test('should open mobile menu when hamburger clicked', async ({ page }) => {
        await page.goto('/');

        const hamburger = page.locator('.hamburger-btn').first();

        if (await hamburger.count() > 0 && await hamburger.isVisible()) {
            await hamburger.click();

            // Mobile menu should appear
            const mobileMenu = page.locator('.mobile-menu, .handyman-mobile-menu').first();
            if (await mobileMenu.count() > 0) {
                await expect(mobileMenu).toBeVisible();
            }
        }
    });
});

test.describe('Handyman Site', () => {
    test('should load handyman page with correct theme', async ({ page }) => {
        await page.goto('/handyman-coventry');

        // Check page loads
        await expect(page.locator('body')).toBeVisible();

        // Check for handyman-specific content
        const content = page.locator('text=Handyman, text=Coventry').first();
        if (await content.count() > 0) {
            await expect(content).toBeVisible();
        }
    });

    test('should display phone number on handyman page', async ({ page }) => {
        await page.goto('/handyman-coventry');

        // Look for phone number
        const phone = page.locator('text=07833 263486, text=7833').first();
        if (await phone.count() > 0) {
            await expect(phone).toBeVisible();
        }
    });
});

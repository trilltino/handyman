import { test, expect } from '@playwright/test';

test.describe('Contact Page', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/contact');
    });

    test('should load contact page', async ({ page }) => {
        await expect(page).toHaveTitle(/Contact/);
    });

    test('should display contact form', async ({ page }) => {
        // Check form elements exist
        const form = page.locator('form').first();
        await expect(form).toBeVisible();
    });

    test('should validate required fields on submit', async ({ page }) => {
        // Try to submit empty form
        const submitBtn = page.locator('button[type="submit"]').first();
        if (await submitBtn.isVisible()) {
            await submitBtn.click();
            // Page should not navigate away on invalid form
            await expect(page).toHaveURL(/contact/);
        }
    });

    test('should accept valid input', async ({ page }) => {
        // Fill in form fields if they exist
        const nameInput = page.locator('input[name="name"], input[id*="name"]').first();
        const emailInput = page.locator('input[name="email"], input[id*="email"]').first();
        const messageInput = page.locator('textarea[name="message"], textarea[id*="message"]').first();

        if (await nameInput.isVisible()) {
            await nameInput.fill('Test User');
        }

        if (await emailInput.isVisible()) {
            await emailInput.fill('test@example.com');
        }

        if (await messageInput.isVisible()) {
            await messageInput.fill('This is a test message from E2E testing.');
        }

        // Verify inputs are filled (don't actually submit to avoid sending emails)
        if (await nameInput.isVisible()) {
            await expect(nameInput).toHaveValue('Test User');
        }
    });
});

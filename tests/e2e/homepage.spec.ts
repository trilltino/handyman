import { test, expect } from '@playwright/test';

test.describe('Homepage', () => {
  test('should load homepage successfully', async ({ page }) => {
    await page.goto('/');

    // Check page title
    await expect(page).toHaveTitle(/Professional Handyman Website Builder/);

    // Check hero section
    await expect(page.locator('h1')).toContainText('Professional Handyman Website Builder');

    // Check navigation
    await expect(page.locator('nav')).toBeVisible();

    // Check CTA buttons
    await expect(page.locator('text=View Pricing')).toBeVisible();
    await expect(page.locator('text=See Example Site')).toBeVisible();
  });

  test('should have working navigation', async ({ page }) => {
    await page.goto('/');

    // Test navigation links
    await page.locator('text=Products').click();
    await expect(page).toHaveURL(/.*products/);

    await page.goto('/');
    await page.locator('text=Packages').click();
    await expect(page).toHaveURL(/.*packages/);

    await page.goto('/');
    await page.locator('text=Contact').click();
    await expect(page).toHaveURL(/.*contact/);
  });

  test('should display testimonials', async ({ page }) => {
    await page.goto('/');

    // Check testimonials section
    await expect(page.locator('text=What Our Customers Say')).toBeVisible();

    // Check testimonial content
    await expect(page.locator('text=Since getting my website')).toBeVisible();
    await expect(page.locator('text=Mike Thompson')).toBeVisible();
  });

  test('should have working newsletter signup', async ({ page }) => {
    await page.goto('/');

    // Check newsletter section
    await expect(page.locator('text=Stay Updated with Industry Tips')).toBeVisible();

    // Test newsletter form
    const emailInput = page.locator('input[type="email"]');
    await expect(emailInput).toBeVisible();

    await emailInput.fill('test@example.com');
    await page.locator('button', { hasText: 'Subscribe' }).click();

    // Check success message (would need to mock API in real implementation)
    // await expect(page.locator('text=Thanks for subscribing')).toBeVisible();
  });

  test('should be responsive on mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/');

    // Check mobile layout
    await expect(page.locator('h1')).toBeVisible();
    await expect(page.locator('nav')).toBeVisible();

    // Check mobile menu if hamburger exists
    // This would depend on the actual mobile menu implementation
  });
});
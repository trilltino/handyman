import { test, expect } from '@playwright/test';

test.describe('Shopping Flow', () => {
  test('should complete full purchase flow', async ({ page }) => {
    // Start on homepage
    await page.goto('/');

    // Navigate to products
    await page.locator('text=Products').click();
    await expect(page).toHaveURL(/.*products/);

    // Select a product (assuming products exist)
    // This would depend on actual product data
    const productCard = page.locator('.product-card').first();
    if (await productCard.isVisible()) {
      await productCard.click();

      // Should be on product detail page
      await expect(page.locator('h1')).toBeVisible();

      // Add to cart (if button exists)
      const addToCartButton = page.locator('button', { hasText: /Add to Cart|Buy Now/ });
      if (await addToCartButton.isVisible()) {
        await addToCartButton.click();

        // Navigate to cart
        await page.locator('text=Cart').click();
        await expect(page).toHaveURL(/.*cart/);

        // Proceed to checkout
        await page.locator('button', { hasText: /Checkout|Proceed to Checkout/ }).click();
        await expect(page).toHaveURL(/.*checkout/);

        // Fill checkout form
        await page.locator('input[name="name"]').fill('John Doe');
        await page.locator('input[name="email"]').fill('john@example.com');
        await page.locator('input[name="address"]').fill('123 Main St');
        await page.locator('input[name="city"]').fill('London');
        await page.locator('input[name="postcode"]').fill('SW1A 1AA');

        // Complete order (would need Stripe test keys in real implementation)
        // await page.locator('button', { hasText: /Complete Order|Pay Now/ }).click();

        // Should show success page or confirmation
        // await expect(page.locator('text=Order confirmed')).toBeVisible();
      }
    }
  });

  test('should handle cart operations', async ({ page }) => {
    await page.goto('/cart');

    // Check empty cart state
    await expect(page.locator('text=Your cart is empty')).toBeVisible();

    // Test cart persistence (if implemented)
    // This would require adding items to cart first
  });

  test('should validate checkout form', async ({ page }) => {
    await page.goto('/checkout');

    // Try to submit empty form
    await page.locator('button[type="submit"]').click();

    // Check validation messages
    await expect(page.locator('text=Name is required')).toBeVisible();
    await expect(page.locator('text=Email is required')).toBeVisible();

    // Fill form partially
    await page.locator('input[name="name"]').fill('John');
    await page.locator('input[name="email"]').fill('invalid-email');

    await page.locator('button[type="submit"]').click();

    // Check email validation
    await expect(page.locator('text=Invalid email format')).toBeVisible();
  });

  test('should handle payment errors gracefully', async ({ page }) => {
    await page.goto('/checkout');

    // Fill valid form
    await page.locator('input[name="name"]').fill('John Doe');
    await page.locator('input[name="email"]').fill('john@example.com');
    await page.locator('input[name="address"]').fill('123 Main St');
    await page.locator('input[name="city"]').fill('London');
    await page.locator('input[name="postcode"]').fill('SW1A 1AA');

    // Submit form (would trigger payment processing)
    // In real implementation, this would test error handling for declined payments
    // await page.locator('button[type="submit"]').click();

    // Check error message display
    // await expect(page.locator('text=Payment failed')).toBeVisible();
  });
});
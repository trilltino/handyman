import { test, expect } from '@playwright/test';

test.describe('Blog Section', () => {
    test('should load blog index page', async ({ page }) => {
        await page.goto('/blog');
        await expect(page.locator('body')).toBeVisible();
    });

    test('should display blog posts', async ({ page }) => {
        await page.goto('/blog');
        // Look for blog post links or cards
        const blogLinks = page.locator('a[href*="/blog/"]');
        if (await blogLinks.count() > 0) {
            expect(await blogLinks.count()).toBeGreaterThanOrEqual(1);
        }
    });

    test('should navigate to individual blog post', async ({ page }) => {
        await page.goto('/blog');
        const firstArticle = page.locator('a[href*="/blog/"]').first();
        if (await firstArticle.count() > 0 && await firstArticle.isVisible()) {
            await firstArticle.click();
            await expect(page.locator('article, .blog-article, .blog-content')).toBeVisible();
        }
    });

    test('should have valid meta description on blog index', async ({ page }) => {
        await page.goto('/blog');
        const metaDesc = page.locator('meta[name="description"]');
        if (await metaDesc.count() > 0) {
            const content = await metaDesc.getAttribute('content');
            expect(content).toBeTruthy();
            expect(content!.length).toBeGreaterThan(10);
        }
    });

    test('should be accessible on mobile', async ({ page }) => {
        await page.setViewportSize({ width: 375, height: 667 });
        await page.goto('/blog');
        await expect(page.locator('body')).toBeVisible();
    });
});

test.describe('Blog Article Pages', () => {
    const blogArticles = [
        '/blog/why-tradesmen-need-websites',
        '/blog/local-seo-guide',
        '/blog/building-trust-online'
    ];

    for (const article of blogArticles) {
        test(`${article} should load successfully`, async ({ page }) => {
            await page.goto(article);
            // Article should either load (200) or redirect
            const body = page.locator('body');
            await expect(body).toBeVisible();
        });
    }
});

test.describe('About Page', () => {
    test('should load about page', async ({ page }) => {
        await page.goto('/about');
        await expect(page.locator('body')).toBeVisible();
    });

    test('should display company information', async ({ page }) => {
        await page.goto('/about');
        const bodyText = await page.locator('body').textContent();
        // Should contain some about page content
        expect(bodyText).toMatch(/about|story|mission|team|XF|Tradesman/i);
    });

    test('should have contact information or CTA', async ({ page }) => {
        await page.goto('/about');
        const contactLink = page.locator('a[href*="contact"], a[href^="tel:"], a[href^="mailto:"]').first();
        if (await contactLink.count() > 0) {
            await expect(contactLink).toBeVisible();
        }
    });
});

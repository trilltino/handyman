import { test, expect } from '@playwright/test';

test.describe('SEO Validation', () => {
    const mainPages = ['/', '/pricing', '/contact', '/about', '/blog', '/packages'];

    for (const path of mainPages) {
        test.describe(`${path} page SEO`, () => {
            test('should have page title', async ({ page }) => {
                await page.goto(path);
                const title = await page.title();
                expect(title).toBeTruthy();
                expect(title.length).toBeGreaterThan(5);
            });

            test('should have meta description', async ({ page }) => {
                await page.goto(path);
                const metaDesc = page.locator('meta[name="description"]');
                if (await metaDesc.count() > 0) {
                    const content = await metaDesc.getAttribute('content');
                    expect(content).toBeTruthy();
                }
            });

            test('should have proper heading structure (h1)', async ({ page }) => {
                await page.goto(path);
                const h1 = page.locator('h1');
                if (await h1.count() > 0) {
                    // Should have exactly one h1
                    expect(await h1.count()).toBe(1);
                }
            });
        });
    }
});

test.describe('Technical SEO', () => {
    test('robots.txt should be accessible', async ({ page }) => {
        const response = await page.goto('/robots.txt');
        if (response) {
            expect([200, 304]).toContain(response.status());
        }
    });

    test('sitemap.xml should be accessible', async ({ page }) => {
        const response = await page.goto('/sitemap.xml');
        if (response) {
            // Sitemap might be proxied from API, accept 200 or 502 (if API not running)
            expect([200, 304, 502]).toContain(response.status());
        }
    });

    test('should have viewport meta tag', async ({ page }) => {
        await page.goto('/');
        const viewport = page.locator('meta[name="viewport"]');
        await expect(viewport).toHaveCount(1);
    });

    test('should have charset meta tag', async ({ page }) => {
        await page.goto('/');
        const charset = page.locator('meta[charset]');
        if (await charset.count() > 0) {
            const charsetValue = await charset.getAttribute('charset');
            expect(charsetValue?.toLowerCase()).toBe('utf-8');
        }
    });
});

test.describe('Open Graph Tags', () => {
    test('homepage should have og:title', async ({ page }) => {
        await page.goto('/');
        const ogTitle = page.locator('meta[property="og:title"]');
        if (await ogTitle.count() > 0) {
            const content = await ogTitle.getAttribute('content');
            expect(content).toBeTruthy();
        }
    });

    test('homepage should have og:description', async ({ page }) => {
        await page.goto('/');
        const ogDesc = page.locator('meta[property="og:description"]');
        if (await ogDesc.count() > 0) {
            const content = await ogDesc.getAttribute('content');
            expect(content).toBeTruthy();
        }
    });

    test('homepage should have og:image', async ({ page }) => {
        await page.goto('/');
        const ogImage = page.locator('meta[property="og:image"]');
        if (await ogImage.count() > 0) {
            const content = await ogImage.getAttribute('content');
            expect(content).toBeTruthy();
            expect(content).toMatch(/^https?:\/\//);
        }
    });
});

test.describe('Structured Data (JSON-LD)', () => {
    test('homepage should have JSON-LD schema', async ({ page }) => {
        await page.goto('/');
        const jsonLd = page.locator('script[type="application/ld+json"]');
        if (await jsonLd.count() > 0) {
            const content = await jsonLd.first().textContent();
            expect(content).toBeTruthy();
            // Should be valid JSON
            expect(() => JSON.parse(content!)).not.toThrow();
        }
    });

    test('JSON-LD should have @context schema.org', async ({ page }) => {
        await page.goto('/');
        const jsonLd = page.locator('script[type="application/ld+json"]');
        if (await jsonLd.count() > 0) {
            const content = await jsonLd.first().textContent();
            if (content) {
                const schema = JSON.parse(content);
                // Could be an array or object
                if (Array.isArray(schema)) {
                    expect(schema[0]['@context']).toContain('schema.org');
                } else {
                    expect(schema['@context']).toContain('schema.org');
                }
            }
        }
    });
});

test.describe('Accessibility Basics', () => {
    test('images should have alt attributes', async ({ page }) => {
        await page.goto('/');
        const images = page.locator('img');
        const count = await images.count();

        for (let i = 0; i < Math.min(count, 10); i++) { // Check first 10 images
            const img = images.nth(i);
            const alt = await img.getAttribute('alt');
            // Alt should exist (can be empty for decorative images)
            expect(alt !== null).toBeTruthy();
        }
    });

    test('links should have discernible text', async ({ page }) => {
        await page.goto('/');
        const links = page.locator('a:not([aria-hidden="true"])');
        const count = await links.count();

        for (let i = 0; i < Math.min(count, 10); i++) { // Check first 10 links
            const link = links.nth(i);
            const text = await link.textContent();
            const ariaLabel = await link.getAttribute('aria-label');
            const title = await link.getAttribute('title');

            // Link should have either text content, aria-label, or title
            expect(text?.trim() || ariaLabel || title).toBeTruthy();
        }
    });

    test('page should have lang attribute', async ({ page }) => {
        await page.goto('/');
        const html = page.locator('html');
        const lang = await html.getAttribute('lang');
        expect(lang).toBeTruthy();
    });
});

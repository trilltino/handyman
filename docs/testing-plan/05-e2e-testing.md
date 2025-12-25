# Phase 5: End-to-End Testing

Browser automation with Playwright.

---

## E2E Testing Setup

### Current Configuration

**File:** `playwright.config.ts`

```bash
# Install Playwright
npm install -D @playwright/test

# Install browsers
npx playwright install
```

### Directory Structure

```
tests/
├── e2e/
│   ├── home.spec.ts        # Home page tests
│   ├── contact.spec.ts     # Contact form tests
│   ├── navigation.spec.ts  # Nav/menu tests
│   └── handyman.spec.ts    # Handyman site tests
```

---

## Test Scenarios

### 1. Home Page

```typescript
// tests/e2e/home.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Home Page', () => {
  test('should load and display hero', async ({ page }) => {
    await page.goto('/');
    
    // Check hero section
    await expect(page.locator('h1')).toBeVisible();
    await expect(page.locator('.hero')).toBeVisible();
  });

  test('should have working navigation', async ({ page }) => {
    await page.goto('/');
    
    // Click pricing link
    await page.click('text=Pricing');
    await expect(page).toHaveURL('/pricing');
  });

  test('should be mobile responsive', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/');
    
    // Mobile menu should be visible
    await expect(page.locator('.hamburger-btn')).toBeVisible();
  });
});
```

### 2. Contact Form

```typescript
// tests/e2e/contact.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Contact Form', () => {
  test('should validate required fields', async ({ page }) => {
    await page.goto('/contact');
    
    // Submit empty form
    await page.click('button[type="submit"]');
    
    // Should show validation errors
    await expect(page.locator('.error')).toBeVisible();
  });

  test('should submit valid form', async ({ page }) => {
    await page.goto('/contact');
    
    // Fill form
    await page.fill('input[name="name"]', 'Test User');
    await page.fill('input[name="email"]', 'test@example.com');
    await page.fill('textarea[name="message"]', 'This is a test message');
    
    // Submit
    await page.click('button[type="submit"]');
    
    // Should show success
    await expect(page.locator('.success')).toBeVisible();
  });

  test('should reject invalid email', async ({ page }) => {
    await page.goto('/contact');
    
    await page.fill('input[name="email"]', 'invalid-email');
    await page.click('button[type="submit"]');
    
    await expect(page.locator('.error')).toContainText('valid email');
  });
});
```

### 3. Mobile Navigation

```typescript
// tests/e2e/navigation.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Mobile Navigation', () => {
  test.beforeEach(async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
  });

  test('should open mobile menu', async ({ page }) => {
    await page.goto('/');
    
    // Click hamburger
    await page.click('.hamburger-btn');
    
    // Menu should be visible
    await expect(page.locator('.mobile-menu')).toBeVisible();
  });

  test('should close menu on link click', async ({ page }) => {
    await page.goto('/');
    
    await page.click('.hamburger-btn');
    await page.click('.mobile-menu a[href="/pricing"]');
    
    // Menu should close
    await expect(page.locator('.mobile-menu')).not.toBeVisible();
    await expect(page).toHaveURL('/pricing');
  });
});
```

### 4. Handyman Site

```typescript
// tests/e2e/handyman.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Handyman Coventry Site', () => {
  test('should load with correct theme', async ({ page }) => {
    await page.goto('/handyman-coventry');
    
    // Check light theme applied
    await expect(page.locator('.handyman-theme')).toBeVisible();
    
    // Check blue header
    const header = page.locator('header');
    await expect(header).toBeVisible();
  });

  test('should display services', async ({ page }) => {
    await page.goto('/handyman-coventry');
    
    // Services section visible
    await expect(page.locator('text=Services')).toBeVisible();
  });

  test('should have phone number visible', async ({ page }) => {
    await page.goto('/handyman-coventry');
    
    await expect(page.locator('text=07833 263486')).toBeVisible();
  });
});
```

---

## Running E2E Tests

```bash
# Run all E2E tests
npx playwright test

# Run specific file
npx playwright test tests/e2e/home.spec.ts

# Run in headed mode (see browser)
npx playwright test --headed

# Run with UI
npx playwright test --ui

# Generate report
npx playwright show-report
```

---

## CI Integration

Add to GitHub Actions:

```yaml
e2e-tests:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    
    - name: Install dependencies
      run: npm ci
    
    - name: Install Playwright
      run: npx playwright install --with-deps
    
    - name: Start server
      run: cargo leptos serve &
      
    - name: Wait for server
      run: sleep 30
      
    - name: Run E2E tests
      run: npx playwright test
```

---

## Implementation Checklist

- [ ] Create `tests/e2e/home.spec.ts`
- [ ] Create `tests/e2e/contact.spec.ts`
- [ ] Create `tests/e2e/navigation.spec.ts`
- [ ] Create `tests/e2e/handyman.spec.ts`
- [ ] Add visual regression tests
- [ ] Add accessibility tests (axe-core)
- [ ] Add to CI pipeline

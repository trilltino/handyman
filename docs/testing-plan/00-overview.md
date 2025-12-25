# XFHandyman Testing Plan

Comprehensive testing strategy for achieving idiomatic Rust testing practices across all crates.

## Documents in This Plan

| File | Description |
|------|-------------|
| [00-overview.md](./00-overview.md) | This file - Testing strategy overview |
| [01-unit-tests.md](./01-unit-tests.md) | Unit testing patterns for each crate |
| [02-integration-tests.md](./02-integration-tests.md) | Integration testing strategy |
| [03-email-testing.md](./03-email-testing.md) | Email service testing & env debugging |
| [04-frontend-testing.md](./04-frontend-testing.md) | Leptos component & SSR testing |
| [05-e2e-testing.md](./05-e2e-testing.md) | End-to-end testing with Playwright |

---

## Current Test Coverage

### Existing Tests Found

| Location | Type | Status |
|----------|------|--------|
| `tests/integration_tests.rs` | Integration | Working |
| `backend/apps/api/tests/api_tests.rs` | Fixtures | Working |
| `backend/apps/api/tests/integration_test.rs` | API Integration | Needs review |
| `tests/e2e/` | E2E | Needs implementation |

### Test Gaps Identified

1. **Unit Tests** - Most modules lack `#[cfg(test)]` blocks
2. **Email Service** - No mock testing, only live SMTP
3. **Shared Crate** - Validation logic untested
4. **Frontend Components** - No component tests
5. **Error Handling** - Edge cases not covered

---

## Crate Testing Priority

### Phase 1: Foundation (shared + lib-core)
- Newtypes & validation
- Model CRUD operations
- Error propagation

### Phase 2: Backend (backend/)
- API handlers
- Email service with mocks
- Auth flows

### Phase 3: Frontend (frontend-leptos)
- SSR rendering
- Component snapshots
- Form validation

### Phase 4: E2E (tests/e2e)
- Full user journeys
- Browser automation
- Visual regression

---

## Running Tests

```bash
# Run all tests
cargo test --all

# Run specific crate tests
cargo test -p lib-core
cargo test -p shared

# Run with logging
RUST_LOG=debug cargo test

# Run E2E tests
npx playwright test
```

# Issue #3: Build Speed Optimization

## Current Build Time
- Docker build: ~15-20 minutes
- Primary bottleneck: Rust compilation

## Build Stages Analysis

| Stage | Time | Notes |
|-------|------|-------|
| Context transfer | ~2 min | 800MB+ transferred |
| Chef recipe | ~1 min | Dependency analysis |
| Dependency build | ~8 min | cargo-chef cached |
| CSS build | ~30 sec | Tailwind v4 |
| API build | ~12 min | Backend binary |
| Leptos build | ~3 min | WASM + SSR |

## Optimization Strategies

### 1. Reduce Docker Context Size

**Current `.dockerignore`:**
Already excludes `target/`, `node_modules/`, etc.

**Additional exclusions:**
```
# Add to .dockerignore
docs/
*.md
!README.md
logs/
*.log
*.txt
.vscode/
.idea/
```

### 2. Improve Cargo Chef Caching

The Dockerfile already uses cargo-chef for dependency caching. This works well when only source code changes (not dependencies).

### 3. Consider Build Caching Services

**Depot.dev** (already in use via Fly.io):
- Fly.io uses Depot for fast Docker builds
- Caches layers across builds
- Should already be working

### 4. Reduce Compilation Complexity

**Type complexity reduction:**
```rust
// Instead of massive tuples in views:
view! {
    <Meta1 />
    <Meta2 />
    <Meta3 />
    // ... 10+ more
}

// Split into sub-components:
view! {
    <SeoMetaTags />
    <OpenGraphTags />
    <StructuredData />
}
```

This reduces type nesting and compile time.

### 5. Parallel Compilation

**Already enabled in Cargo.toml:**
```toml
[profile.release]
lto = "thin"  # Faster than "fat"
codegen-units = 1  # Single codegen for optimization
```

### 6. Enable Incremental Compilation in Docker

Add to Dockerfile:
```dockerfile
ENV CARGO_INCREMENTAL=1
```

Note: Only helps for subsequent builds in same builder.

---

## Recommended Quick Wins

### Priority 1: Already Implemented ✅
- cargo-chef for dependency caching
- Depot.dev via Fly.io
- Multi-stage Docker build

### Priority 2: Easy Improvements
- [ ] Update `.dockerignore` to exclude docs
- [ ] Split large view components

### Priority 3: Future Optimizations
- [ ] Consider GitHub Actions self-hosted runner
- [ ] Pre-built Docker base image with dependencies
- [ ] Build-time feature flags to skip unused code

---

## Expected Improvements

| Optimization | Time Saved | Effort |
|-------------|-----------|--------|
| Better .dockerignore | 1-2 min | Low |
| Split components | 2-3 min | Medium |
| Pre-built base | 5+ min | High |

## Status: ⏳ OPTIONAL
Focus on fixing build first, then optimize.

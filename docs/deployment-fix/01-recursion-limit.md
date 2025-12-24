# Issue #1: Recursion Limit Overflow

## Error Message
```
error: queries overflow the depth limit!
  = help: consider increasing the recursion limit by adding a 
    `#![recursion_limit = "256"]` attribute to your crate (`frontend_leptos`)
  = note: query depth increased by 130 when computing layout of `{async block...}`
```

## Root Cause Analysis

### Why This Happens
Leptos generates complex nested types for view hierarchies. Each component, attribute, and child element contributes to type nesting. When you have:
- Many meta tags (SEO components)
- Deeply nested routes
- Complex form components

The type system creates extremely deep recursive types during compilation.

### Type Complexity Example
The error shows types like:
```rust
leptos::into_view::View<(
  leptos_meta::title::TitleView, 
  leptos_meta::RegisteredMetaTag<...>,
  leptos_meta::RegisteredMetaTag<...>,
  // ... 10+ more meta tags
  tachys::html::element::HtmlElement<...>
)>
```

Each nested element adds ~10-20 to the type depth.

## Solution

### Files Modified
1. `frontend-leptos/src/lib.rs` - For WASM/hydrate build
2. `frontend-leptos/src/main.rs` - For SSR binary build

### Code Changes

**lib.rs:**
```rust
// At the top of the file, after doc comments
#![recursion_limit = "1024"]
```

**main.rs:**
```rust
// At the top of the file, after doc comments
#![recursion_limit = "1024"]
```

## Why 1024?
- Default limit: 128
- Error suggests 256, but that's minimum
- Complex Leptos apps often need 512-2048
- 1024 provides headroom for future growth

## Prevention
To reduce type complexity in the future:
1. Split large components into smaller ones
2. Use `view! {}` fragments instead of deeply nested elements
3. Consider using `#[component]` for repeated patterns

## Status: ✅ FIXED
Commits: `7ab7dc9`, `1157e7a`

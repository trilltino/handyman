# Shared Types

Cross-stack types for frontend and backend.

## Purpose

Types defined here are compiled twice:
- Once for WASM (frontend)
- Once for native (backend)

This ensures type safety across the full stack.

## Structure

```
shared/
├── lib.rs           # Module exports
├── error.rs         # SharedError enum
├── metadata.rs      # SEO PageMetadata
├── validation.rs    # Validate trait
├── schema/          # JSON-LD generators
└── types/
    ├── api.rs       # ApiResponse<T>
    ├── contact.rs   # ContactForm
    └── product.rs   # Product types
```

## Key Types

### ApiResponse<T>
```rust
let resp = ApiResponse::success("Created", data);
let resp = ApiResponse::error("Failed");
```

### Validate Trait
```rust
impl Validate for ContactForm {
    fn validate(&self) -> Result<(), Cow<'static, str>> {
        if self.email.is_empty() {
            return Err("Email required".into());
        }
        Ok(())
    }
}
```

## Adding New Types

1. Create in `types/your_type.rs`
2. Implement `Validate` in `validation.rs`
3. Export in `types/mod.rs` and `lib.rs`

See [IDM/03_shared_types.md](../IDM/03_shared_types.md) for details.

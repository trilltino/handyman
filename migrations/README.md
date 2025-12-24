# Database Migrations

Production-ready database schema for XF Tradesmen.

## Quick Start

### Fresh Database (Recommended)
```bash
# Run the init schema
psql $DATABASE_URL -f migrations/20241224000000_init.sql
```

### Using SQLx Migrations
```bash
# Set DATABASE_URL
export DATABASE_URL="postgres://user:pass@localhost/xftradesmen"

# Run migrations
sqlx migrate run
```

## Files

| File | Purpose |
|------|---------|
| `20241224000000_init.sql` | **Main schema** - All tables, indexes, triggers |
| `20241224000001_rollback.sql` | Rollback - Drops everything (destructive!) |
| `_archive/` | Old individual migrations (kept for history) |

## Schema Overview

```
users                 - Handymen and admins
customers             - Service recipients
contact_submissions   - Website contact form
bookings              - Job appointments
quotes                - Itemized quotes
quote_templates       - Reusable templates
availability          - Handyman schedule
job_notes             - Job photos/notes
orders                - Product orders
order_items           - Order line items
blog_posts            - Blog articles
newsletter_subscribers - Email list
```

## Key Features

- **ENUMs** for status fields (type-safe)
- **TIMESTAMPTZ** for all timestamps (timezone-aware)
- **JSONB** for flexible address/items storage
- **Proper indexes** on foreign keys and query patterns
- **Auto-update triggers** for `updated_at` columns
- **Constraints** for data integrity (prices >= 0, ratings 1-5)

## Rollback

```bash
# WARNING: This destroys all data!
psql $DATABASE_URL -f migrations/20241224000001_rollback.sql
```

## Adding New Migrations

For schema changes after deployment:

```bash
# Create timestamped migration
sqlx migrate add <name>

# Example
sqlx migrate add add_payment_method_to_orders
```

## Production Checklist

- [ ] Run on staging first
- [ ] Backup production database
- [ ] Run migrations during low-traffic period
- [ ] Verify schema after migration
- [ ] Test critical queries

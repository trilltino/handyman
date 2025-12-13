# Database Migrations

This directory contains SQL migrations managed by `sqlx`.

## Running Migrations

```bash
# Using justfile
just migrate

# Or directly with sqlx
sqlx migrate run
```

## Creating New Migrations

```bash
sqlx migrate add <migration_name>
```

This will create a new migration file in the format:
```
<timestamp>_<migration_name>.sql
```

## Migration Files

Migrations are applied in order based on their timestamp prefix. Each migration should be:
- **Idempotent**: Safe to run multiple times
- **Reversible**: Include a corresponding down migration if needed
- **Atomic**: Complete successfully or roll back entirely

## Current Migrations

- `20251202104220_create_contact_table.sql` - Initial contact submissions table

## Environment Variables

Migrations require the `SERVICE_DB_URL` environment variable:

```bash
SERVICE_DB_URL=postgresql://postgres:password@localhost:5432/handyman
```

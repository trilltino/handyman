-- ============================================================================
-- XF Tradesmen - Database Schema Rollback
-- ============================================================================
-- Completely drops all tables, types, and functions
-- WARNING: This will destroy ALL DATA
-- ============================================================================

-- Drop triggers first
DROP TRIGGER IF EXISTS update_users_timestamp ON users;
DROP TRIGGER IF EXISTS update_customers_timestamp ON customers;
DROP TRIGGER IF EXISTS update_bookings_timestamp ON bookings;
DROP TRIGGER IF EXISTS update_quotes_timestamp ON quotes;
DROP TRIGGER IF EXISTS update_quote_templates_timestamp ON quote_templates;
DROP TRIGGER IF EXISTS update_orders_timestamp ON orders;
DROP TRIGGER IF EXISTS update_blog_posts_timestamp ON blog_posts;

-- Drop tables in reverse dependency order
DROP TABLE IF EXISTS job_notes CASCADE;
DROP TABLE IF EXISTS availability CASCADE;
DROP TABLE IF EXISTS order_items CASCADE;
DROP TABLE IF EXISTS orders CASCADE;
DROP TABLE IF EXISTS newsletter_subscribers CASCADE;
DROP TABLE IF EXISTS blog_posts CASCADE;
DROP TABLE IF EXISTS quote_templates CASCADE;
DROP TABLE IF EXISTS quotes CASCADE;
DROP TABLE IF EXISTS bookings CASCADE;
DROP TABLE IF EXISTS customers CASCADE;
DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS contact_submissions CASCADE;

-- Drop custom types
DROP TYPE IF EXISTS blog_post_status CASCADE;
DROP TYPE IF EXISTS order_status CASCADE;
DROP TYPE IF EXISTS booking_status CASCADE;
DROP TYPE IF EXISTS quote_status CASCADE;

-- Drop functions
DROP FUNCTION IF EXISTS update_updated_at_column() CASCADE;

-- Drop extensions (optional - usually keep these)
-- DROP EXTENSION IF EXISTS "uuid-ossp";

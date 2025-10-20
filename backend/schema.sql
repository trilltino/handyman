-- ============================================================================
-- Handyman Marketplace Database Schema
-- ============================================================================
--
-- This SQL script defines the complete database schema for the handyman marketplace.
-- It creates all tables, indexes, and triggers needed for the application.
--
-- Tables:
-- 1. users            - Handyman user accounts (authentication)
-- 2. customers        - Customer contact information (from bookings)
-- 3. bookings         - Service booking requests with payment tracking
-- 4. contact_messages - Contact form submissions
--
-- How to run:
-- 1. Create database: cargo run --bin create_db
-- 2. Initialize schema: cargo run --bin init_db
--    OR manually: psql -U postgres -d handyman -f backend/schema.sql
--
-- ============================================================================

-- Drop existing tables (in correct order due to foreign key constraints)
-- CASCADE ensures dependent objects (foreign keys, triggers) are dropped too
DROP TABLE IF EXISTS bookings CASCADE;         -- Has foreign keys to users and customers
DROP TABLE IF EXISTS customers CASCADE;        -- Referenced by bookings
DROP TABLE IF EXISTS contact_messages CASCADE; -- Standalone table
DROP TABLE IF EXISTS users CASCADE;            -- Referenced by bookings

-- ============================================================================
-- Users Table - Handyman accounts
-- ============================================================================
-- Stores handyman user accounts for authentication and authorization.
-- Currently: Only handymen can register and login
-- Future: Could extend to include customer accounts
--
-- Security:
-- - Passwords are hashed with HMAC-SHA256 before storage
-- - token_salt (UUID) is used for JWT signature generation
-- - username and email must be unique
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,                      -- Auto-incrementing user ID
    username VARCHAR(255) UNIQUE NOT NULL,      -- Unique username for login
    email VARCHAR(255) UNIQUE NOT NULL,         -- Unique email address
    pwd_hash TEXT NOT NULL,                     -- HMAC-SHA256 password hash (never store plain passwords!)
    token_salt UUID NOT NULL,                   -- Random UUID for JWT token generation (invalidates tokens on change)
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,  -- Account creation timestamp
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP   -- Last update timestamp (auto-updated by trigger)
);

-- ============================================================================
-- Customers Table - Service booking customer information
-- ============================================================================
-- Stores customer contact details collected from booking forms.
-- Customers don't have accounts (no passwords) - just contact info.
--
-- Deduplication:
-- - email is unique to prevent duplicate customer records
-- - Repository uses "create_or_get" pattern to reuse existing customers
CREATE TABLE IF NOT EXISTS customers (
    id SERIAL PRIMARY KEY,                      -- Auto-incrementing customer ID
    full_name VARCHAR(255) NOT NULL,            -- Customer's full name
    email VARCHAR(255) UNIQUE NOT NULL,         -- Email (unique to prevent duplicates)
    phone VARCHAR(50),                          -- Phone number (optional)
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,  -- First booking timestamp
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP   -- Last update (auto-updated by trigger)
);

-- ============================================================================
-- Bookings Table - Service booking requests
-- ============================================================================
-- Stores all service booking requests with payment and status tracking.
--
-- Workflow:
-- 1. Customer fills out booking form (frontend)
-- 2. Backend creates customer (or finds existing) and calculates price
-- 3. Booking created with 'pending' status
-- 4. Stripe payment integration (future: payment_intent_id, payment_method_id)
-- 5. Status updates: pending → confirmed → completed
--
-- Pricing (see handlers/booking.rs):
-- - Plumbing: €150, Electrical: €180, Carpentry: €120
-- - Painting: €100, General Repair: €80, Other: €100
CREATE TABLE IF NOT EXISTS bookings (
    id SERIAL PRIMARY KEY,                                   -- Auto-incrementing booking ID
    customer_id INTEGER REFERENCES customers(id),            -- Foreign key to customers table
    user_id INTEGER REFERENCES users(id),                    -- Foreign key to users (handyman assigned to job)
    work_type VARCHAR(50) NOT NULL,                          -- Type: plumbing, electrical, carpentry, painting, general, other
    location TEXT NOT NULL,                                  -- Service location address
    description TEXT,                                        -- Optional description of work needed
    price_cents INTEGER NOT NULL,                            -- Price in cents (€150 = 15000 cents) - avoids float precision issues
    status VARCHAR(50) DEFAULT 'pending',                    -- Booking status: pending, confirmed, in_progress, completed, cancelled
    payment_status VARCHAR(50) DEFAULT 'pending',            -- Payment status: pending, paid, failed, refunded
    payment_intent_id TEXT,                                  -- Stripe PaymentIntent ID (for payment tracking)
    payment_method_id TEXT,                                  -- Stripe PaymentMethod ID (credit card, etc.)
    scheduled_date TIMESTAMP WITH TIME ZONE,                 -- When service is scheduled (optional)
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,  -- Booking creation timestamp
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP   -- Last update (auto-updated by trigger)
);

-- ============================================================================
-- Contact Messages Table - Contact form submissions
-- ============================================================================
-- Stores messages submitted through the contact form.
-- No authentication required - anyone can submit a contact message.
--
-- Future enhancements:
-- - Add status field (new, read, responded, archived)
-- - Add response tracking
-- - Add email notification system
CREATE TABLE IF NOT EXISTS contact_messages (
    id SERIAL PRIMARY KEY,                                   -- Auto-incrementing message ID
    name VARCHAR(255) NOT NULL,                              -- Sender's name
    email VARCHAR(255) NOT NULL,                             -- Sender's email (for replies)
    message TEXT NOT NULL,                                   -- Message content
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP  -- Submission timestamp
);

-- ============================================================================
-- Indexes - Performance optimization for common queries
-- ============================================================================
-- Indexes speed up queries by creating fast lookup structures.
-- Trade-off: Faster reads, slightly slower writes (index must be updated on INSERT/UPDATE)
CREATE INDEX IF NOT EXISTS idx_bookings_customer ON bookings(customer_id);  -- Speed up: JOIN bookings with customers
CREATE INDEX IF NOT EXISTS idx_bookings_user ON bookings(user_id);         -- Speed up: Find all bookings for a handyman
CREATE INDEX IF NOT EXISTS idx_bookings_status ON bookings(status);        -- Speed up: Filter by status (pending, completed, etc.)
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);          -- Speed up: Login queries by username
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);                -- Speed up: Find user by email

-- ============================================================================
-- Triggers - Automatic timestamp updates
-- ============================================================================
-- These triggers automatically update the 'updated_at' column whenever a row is modified.
-- This ensures accurate tracking of when records were last changed.
-- Trigger function: Sets updated_at to current timestamp
-- Called automatically before any UPDATE operation on tables with triggers
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;  -- Set updated_at to now
    RETURN NEW;                          -- Return modified row
END;
$$ language 'plpgsql';

-- Attach trigger to users table: Auto-update updated_at on any UPDATE
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Attach trigger to customers table: Auto-update updated_at on any UPDATE
CREATE TRIGGER update_customers_updated_at BEFORE UPDATE ON customers
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Attach trigger to bookings table: Auto-update updated_at on any UPDATE
-- Example: When booking status changes from 'pending' to 'confirmed', updated_at is automatically set
CREATE TRIGGER update_bookings_updated_at BEFORE UPDATE ON bookings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- Schema initialization complete!
-- ============================================================================

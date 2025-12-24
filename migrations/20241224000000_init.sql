-- ============================================================================
-- XF Tradesmen - Consolidated Database Schema
-- ============================================================================
-- Version: 1.0.0
-- This is an optimized, consolidated schema for fresh deployments.
-- For existing databases, use the individual migration files.
-- ============================================================================

-- ============================================================================
-- Extensions
-- ============================================================================
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ============================================================================
-- Utility Functions
-- ============================================================================

-- Auto-update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- Custom Types
-- ============================================================================
DO $$ BEGIN
    CREATE TYPE blog_post_status AS ENUM ('draft', 'published', 'archived');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE order_status AS ENUM ('pending', 'processing', 'completed', 'cancelled', 'refunded');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE booking_status AS ENUM ('pending', 'confirmed', 'in_progress', 'completed', 'cancelled');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE quote_status AS ENUM ('draft', 'sent', 'viewed', 'accepted', 'rejected', 'expired');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- ============================================================================
-- Users Table (for handymen/admins)
-- ============================================================================
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    phone VARCHAR(20),
    role VARCHAR(20) NOT NULL DEFAULT 'user',   -- admin, handyman, user
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_role ON users(role);

-- ============================================================================
-- Customers Table (service recipients)
-- ============================================================================
CREATE TABLE IF NOT EXISTS customers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    phone VARCHAR(20),
    addresses JSONB DEFAULT '[]',               -- Array of {line1, line2, city, postcode}
    notes TEXT,                                 -- Internal notes
    tags VARCHAR(50)[],                         -- Tags: vip, subscriber, etc.
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_customers_email ON customers(email);
CREATE INDEX IF NOT EXISTS idx_customers_name ON customers(name);

-- ============================================================================
-- Contact Submissions
-- ============================================================================
CREATE TABLE IF NOT EXISTS contact_submissions (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    subject VARCHAR(500),
    message TEXT NOT NULL,
    ip_address VARCHAR(45),
    user_agent TEXT,
    submitted_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT email_format CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$')
);

CREATE INDEX IF NOT EXISTS idx_contact_submissions_email ON contact_submissions(email);
CREATE INDEX IF NOT EXISTS idx_contact_submissions_submitted_at ON contact_submissions(submitted_at DESC);

-- ============================================================================
-- Bookings Table
-- ============================================================================
CREATE TABLE IF NOT EXISTS bookings (
    id SERIAL PRIMARY KEY,
    customer_id INTEGER REFERENCES customers(id),
    user_id INTEGER REFERENCES users(id),       -- Assigned handyman
    
    -- Job details
    service_type VARCHAR(50) NOT NULL,          -- plumbing, electrical, etc.
    title VARCHAR(255),
    description TEXT,
    
    -- Scheduling
    scheduled_date DATE,
    scheduled_time TIME,
    estimated_duration INTEGER,                 -- Minutes
    actual_duration INTEGER,                    -- Minutes
    
    -- Status tracking
    status booking_status DEFAULT 'pending',
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    
    -- Pricing
    quote_id INTEGER,                           -- Reference added after quotes table
    total_cents INTEGER,
    
    -- Customer feedback
    customer_rating INTEGER CHECK (customer_rating BETWEEN 1 AND 5),
    customer_review TEXT,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_bookings_customer ON bookings(customer_id);
CREATE INDEX IF NOT EXISTS idx_bookings_user ON bookings(user_id);
CREATE INDEX IF NOT EXISTS idx_bookings_status ON bookings(status);
CREATE INDEX IF NOT EXISTS idx_bookings_scheduled ON bookings(scheduled_date, scheduled_time);

-- ============================================================================
-- Quotes Table
-- ============================================================================
CREATE TABLE IF NOT EXISTS quotes (
    id SERIAL PRIMARY KEY,
    customer_id INTEGER REFERENCES customers(id),
    
    -- Quote details
    title VARCHAR(255) NOT NULL,
    items JSONB NOT NULL DEFAULT '[]',          -- [{description, quantity, unit_price, total}]
    subtotal_cents INTEGER NOT NULL DEFAULT 0,
    discount_cents INTEGER NOT NULL DEFAULT 0,
    total_cents INTEGER NOT NULL DEFAULT 0,
    
    -- Validity
    valid_until DATE,
    status quote_status DEFAULT 'draft',
    
    -- Response
    customer_notes TEXT,
    accepted_at TIMESTAMPTZ,
    booking_id INTEGER REFERENCES bookings(id),
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_quotes_customer ON quotes(customer_id);
CREATE INDEX IF NOT EXISTS idx_quotes_status ON quotes(status);
CREATE INDEX IF NOT EXISTS idx_quotes_valid_until ON quotes(valid_until);

-- Add quote reference to bookings (circular reference)
ALTER TABLE bookings ADD CONSTRAINT fk_bookings_quote 
    FOREIGN KEY (quote_id) REFERENCES quotes(id);

-- ============================================================================
-- Quote Templates
-- ============================================================================
CREATE TABLE IF NOT EXISTS quote_templates (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    service_type VARCHAR(50) NOT NULL,
    items JSONB NOT NULL DEFAULT '[]',
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_quote_templates_service ON quote_templates(service_type);

-- ============================================================================
-- Availability (Handyman Schedule)
-- ============================================================================
CREATE TABLE IF NOT EXISTS availability (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id) NOT NULL,
    date DATE NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    is_available BOOLEAN DEFAULT true,
    reason VARCHAR(100),                        -- holiday, personal, booked
    booking_id INTEGER REFERENCES bookings(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT no_overlap UNIQUE(user_id, date, start_time),
    CONSTRAINT valid_time_range CHECK (end_time > start_time)
);

CREATE INDEX IF NOT EXISTS idx_availability_lookup ON availability(user_id, date, is_available);

-- ============================================================================
-- Job Notes (Photos, Notes for Jobs)
-- ============================================================================
CREATE TABLE IF NOT EXISTS job_notes (
    id SERIAL PRIMARY KEY,
    booking_id INTEGER REFERENCES bookings(id) NOT NULL,
    note_type VARCHAR(20) NOT NULL DEFAULT 'note',  -- note, before_photo, after_photo, material
    content TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_job_notes_booking ON job_notes(booking_id);

-- ============================================================================
-- Orders Table (for product sales if needed)
-- ============================================================================
CREATE TABLE IF NOT EXISTS orders (
    id SERIAL PRIMARY KEY,
    order_number VARCHAR(50) UNIQUE NOT NULL,
    customer_id INTEGER REFERENCES customers(id),
    customer_email VARCHAR(255) NOT NULL,
    customer_name VARCHAR(255),
    billing_address JSONB,
    shipping_address JSONB,
    subtotal_cents INTEGER NOT NULL CHECK (subtotal_cents >= 0),
    tax_cents INTEGER DEFAULT 0 CHECK (tax_cents >= 0),
    shipping_cents INTEGER DEFAULT 0 CHECK (shipping_cents >= 0),
    total_cents INTEGER NOT NULL CHECK (total_cents >= 0),
    status order_status DEFAULT 'pending',
    payment_intent_id VARCHAR(255),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_orders_number ON orders(order_number);
CREATE INDEX IF NOT EXISTS idx_orders_customer ON orders(customer_id);
CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status);

-- ============================================================================
-- Order Items
-- ============================================================================
CREATE TABLE IF NOT EXISTS order_items (
    id SERIAL PRIMARY KEY,
    order_id INTEGER REFERENCES orders(id) ON DELETE CASCADE NOT NULL,
    description VARCHAR(255) NOT NULL,          -- Item description (snapshot)
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    unit_price_cents INTEGER NOT NULL CHECK (unit_price_cents >= 0),
    total_cents INTEGER NOT NULL CHECK (total_cents >= 0),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_order_items_order ON order_items(order_id);

-- ============================================================================
-- Blog Posts
-- ============================================================================
CREATE TABLE IF NOT EXISTS blog_posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    content TEXT NOT NULL,
    excerpt TEXT,
    author VARCHAR(100) NOT NULL,
    status blog_post_status DEFAULT 'draft',
    seo_title VARCHAR(255),
    seo_description TEXT,
    featured_image VARCHAR(500),
    tags VARCHAR(50)[],
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_blog_posts_slug ON blog_posts(slug);
CREATE INDEX IF NOT EXISTS idx_blog_posts_status ON blog_posts(status);
CREATE INDEX IF NOT EXISTS idx_blog_posts_published ON blog_posts(published_at DESC) WHERE status = 'published';

-- ============================================================================
-- Newsletter Subscribers
-- ============================================================================
CREATE TABLE IF NOT EXISTS newsletter_subscribers (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    is_active BOOLEAN DEFAULT true,
    subscribed_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_newsletter_active ON newsletter_subscribers(is_active) WHERE is_active = true;

-- ============================================================================
-- Auto-update Triggers
-- ============================================================================
CREATE TRIGGER update_users_timestamp BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_customers_timestamp BEFORE UPDATE ON customers
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_bookings_timestamp BEFORE UPDATE ON bookings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_quotes_timestamp BEFORE UPDATE ON quotes
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_quote_templates_timestamp BEFORE UPDATE ON quote_templates
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_orders_timestamp BEFORE UPDATE ON orders
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_blog_posts_timestamp BEFORE UPDATE ON blog_posts
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- Seed Data
-- ============================================================================

-- Quote Templates
INSERT INTO quote_templates (name, service_type, items) VALUES
    ('Leaky Tap Repair', 'plumbing', '[{"description": "Call-out fee", "quantity": 1, "unit_price": 3000}, {"description": "Labour (1 hour)", "quantity": 1, "unit_price": 4500}, {"description": "Materials", "quantity": 1, "unit_price": 500}]'),
    ('Light Fitting Installation', 'electrical', '[{"description": "Call-out fee", "quantity": 1, "unit_price": 3000}, {"description": "Labour (1 hour)", "quantity": 1, "unit_price": 5000}, {"description": "Standard light fitting", "quantity": 1, "unit_price": 2500}]'),
    ('IKEA Furniture Assembly (Small)', 'assembly', '[{"description": "Assembly service", "quantity": 1, "unit_price": 4500}]'),
    ('IKEA Furniture Assembly (Large)', 'assembly', '[{"description": "Assembly service (large item)", "quantity": 1, "unit_price": 8500}]'),
    ('Door Hanging', 'carpentry', '[{"description": "Call-out fee", "quantity": 1, "unit_price": 3000}, {"description": "Door hanging labour", "quantity": 1, "unit_price": 7500}]'),
    ('Shelf Installation', 'carpentry', '[{"description": "Shelf installation", "quantity": 1, "unit_price": 3500}]'),
    ('TV Wall Mount', 'general', '[{"description": "TV mounting service", "quantity": 1, "unit_price": 6500}]'),
    ('Picture Hanging (up to 5)', 'general', '[{"description": "Picture hanging service", "quantity": 1, "unit_price": 4000}]')
ON CONFLICT DO NOTHING;

-- ============================================================================
-- Comments
-- ============================================================================
COMMENT ON TABLE users IS 'Handymen and admin users';
COMMENT ON TABLE customers IS 'Service recipients/customers';
COMMENT ON TABLE contact_submissions IS 'Website contact form submissions';
COMMENT ON TABLE bookings IS 'Job bookings/appointments';
COMMENT ON TABLE quotes IS 'Itemized quotes for jobs';
COMMENT ON TABLE quote_templates IS 'Reusable quote templates';
COMMENT ON TABLE availability IS 'Handyman availability calendar';
COMMENT ON TABLE job_notes IS 'Notes and photos for jobs';
COMMENT ON TABLE orders IS 'Product/service orders';
COMMENT ON TABLE order_items IS 'Line items for orders';
COMMENT ON TABLE blog_posts IS 'Blog articles';
COMMENT ON TABLE newsletter_subscribers IS 'Newsletter email list';

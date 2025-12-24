-- ============================================================================
-- Quote System Migration
-- ============================================================================
-- Adds quotes table for the handyman quote builder feature
-- Allows handymen to create itemized quotes and send to customers

-- ============================================================================
-- Quotes Table - Itemized quotes for jobs
-- ============================================================================
CREATE TABLE IF NOT EXISTS quotes (
    id SERIAL PRIMARY KEY,
    customer_id INTEGER REFERENCES customers(id),
    
    -- Quote details
    title VARCHAR(255) NOT NULL,                              -- Brief title for the quote
    items JSONB NOT NULL DEFAULT '[]',                        -- Array of line items [{description, quantity, unit_price}]
    subtotal_cents INTEGER NOT NULL DEFAULT 0,                -- Subtotal before any adjustments
    discount_cents INTEGER NOT NULL DEFAULT 0,                -- Any discount applied
    total_cents INTEGER NOT NULL DEFAULT 0,                   -- Final total in cents
    
    -- Validity & Status
    valid_until DATE,                                         -- Quote expiry date
    status VARCHAR(20) NOT NULL DEFAULT 'draft',              -- draft, sent, viewed, accepted, rejected, expired
    
    -- Customer response
    customer_notes TEXT,                                      -- Notes from customer when responding
    accepted_at TIMESTAMP WITH TIME ZONE,                     -- When customer accepted
    
    -- Linked job (when quote is converted to booking)
    booking_id INTEGER REFERENCES bookings(id),               -- Links to created booking when accepted
    
    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- Quote Templates Table - Reusable quote templates
-- ============================================================================
CREATE TABLE IF NOT EXISTS quote_templates (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,                               -- Template name
    service_type VARCHAR(50) NOT NULL,                        -- plumbing, electrical, etc.
    items JSONB NOT NULL DEFAULT '[]',                        -- Default line items
    is_active BOOLEAN DEFAULT true,                           -- Soft delete
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- Availability Table - Handyman calendar/schedule
-- ============================================================================
CREATE TABLE IF NOT EXISTS availability (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    
    -- Time slot
    date DATE NOT NULL,                                       -- Date of availability
    start_time TIME NOT NULL,                                 -- Start time
    end_time TIME NOT NULL,                                   -- End time
    
    -- Status
    is_available BOOLEAN DEFAULT true,                        -- false = blocked out
    reason VARCHAR(100),                                      -- "holiday", "personal", "booked"
    booking_id INTEGER REFERENCES bookings(id),               -- If blocked due to a booking
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    -- Prevent double-booking
    UNIQUE(user_id, date, start_time)
);

-- ============================================================================
-- Job Notes Table - Notes and photos for jobs
-- ============================================================================
CREATE TABLE IF NOT EXISTS job_notes (
    id SERIAL PRIMARY KEY,
    booking_id INTEGER REFERENCES bookings(id) NOT NULL,
    
    note_type VARCHAR(20) NOT NULL DEFAULT 'note',            -- note, before_photo, after_photo, material
    content TEXT,                                             -- Note text or file URL
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- Enhance Customers Table - Add address fields
-- ============================================================================
ALTER TABLE customers 
    ADD COLUMN IF NOT EXISTS addresses JSONB DEFAULT '[]',    -- Array of addresses
    ADD COLUMN IF NOT EXISTS notes TEXT,                      -- Internal notes about customer
    ADD COLUMN IF NOT EXISTS tags VARCHAR(100)[];             -- Tags like "vip", "subscriber"

-- ============================================================================
-- Enhance Bookings Table - Add more tracking fields
-- ============================================================================
ALTER TABLE bookings
    ADD COLUMN IF NOT EXISTS quote_id INTEGER REFERENCES quotes(id),
    ADD COLUMN IF NOT EXISTS estimated_duration INTEGER,      -- Duration in minutes
    ADD COLUMN IF NOT EXISTS actual_duration INTEGER,         -- Actual time taken
    ADD COLUMN IF NOT EXISTS started_at TIMESTAMP WITH TIME ZONE,
    ADD COLUMN IF NOT EXISTS completed_at TIMESTAMP WITH TIME ZONE,
    ADD COLUMN IF NOT EXISTS customer_rating INTEGER,         -- 1-5 rating
    ADD COLUMN IF NOT EXISTS customer_review TEXT;            -- Review text

-- ============================================================================
-- Indexes for new tables
-- ============================================================================
CREATE INDEX IF NOT EXISTS idx_quotes_customer ON quotes(customer_id);
CREATE INDEX IF NOT EXISTS idx_quotes_status ON quotes(status);
CREATE INDEX IF NOT EXISTS idx_availability_user_date ON availability(user_id, date);
CREATE INDEX IF NOT EXISTS idx_job_notes_booking ON job_notes(booking_id);

-- ============================================================================
-- Triggers for updated_at
-- ============================================================================
CREATE TRIGGER update_quotes_updated_at BEFORE UPDATE ON quotes
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_quote_templates_updated_at BEFORE UPDATE ON quote_templates
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- Seed Quote Templates
-- ============================================================================
INSERT INTO quote_templates (name, service_type, items) VALUES
    ('Leaky Tap Repair', 'plumbing', '[{"description": "Call-out fee", "quantity": 1, "unit_price": 3000}, {"description": "Labour (1 hour)", "quantity": 1, "unit_price": 4500}, {"description": "Materials", "quantity": 1, "unit_price": 500}]'),
    ('Light Fitting Installation', 'electrical', '[{"description": "Call-out fee", "quantity": 1, "unit_price": 3000}, {"description": "Labour (1 hour)", "quantity": 1, "unit_price": 5000}, {"description": "Standard light fitting", "quantity": 1, "unit_price": 2500}]'),
    ('IKEA Furniture Assembly (Small)', 'assembly', '[{"description": "Assembly service", "quantity": 1, "unit_price": 4500}]'),
    ('IKEA Furniture Assembly (Large)', 'assembly', '[{"description": "Assembly service (large item)", "quantity": 1, "unit_price": 8500}]'),
    ('Door Hanging', 'carpentry', '[{"description": "Call-out fee", "quantity": 1, "unit_price": 3000}, {"description": "Door hanging labour", "quantity": 1, "unit_price": 7500}]'),
    ('Shelf Installation (per shelf)', 'carpentry', '[{"description": "Shelf installation", "quantity": 1, "unit_price": 3500}]'),
    ('TV Wall Mount', 'general', '[{"description": "TV mounting service", "quantity": 1, "unit_price": 6500}]'),
    ('Picture Hanging (up to 5)', 'general', '[{"description": "Picture hanging service", "quantity": 1, "unit_price": 4000}]')
ON CONFLICT DO NOTHING;

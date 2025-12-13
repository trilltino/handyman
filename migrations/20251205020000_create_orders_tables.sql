-- Orders and Order Items Tables
-- Stores order information for the e-commerce platform

CREATE TABLE IF NOT EXISTS orders (
    id SERIAL PRIMARY KEY,
    order_number VARCHAR(50) UNIQUE NOT NULL,
    customer_email VARCHAR(255) NOT NULL,
    customer_name VARCHAR(255),
    billing_address JSONB,
    shipping_address JSONB,
    subtotal DECIMAL(10,2) NOT NULL CHECK (subtotal >= 0),
    tax_amount DECIMAL(10,2) DEFAULT 0 CHECK (tax_amount >= 0),
    shipping_amount DECIMAL(10,2) DEFAULT 0 CHECK (shipping_amount >= 0),
    total DECIMAL(10,2) NOT NULL CHECK (total >= 0),
    status VARCHAR(50) DEFAULT 'pending' CHECK (status IN ('pending', 'processing', 'completed', 'cancelled', 'refunded')),
    payment_intent_id VARCHAR(255),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Order Items Table
-- Stores individual items within each order

CREATE TABLE IF NOT EXISTS order_items (
    id SERIAL PRIMARY KEY,
    order_id INTEGER NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id INTEGER NOT NULL REFERENCES products(id),
    product_name VARCHAR(255) NOT NULL, -- Snapshot of product name at time of order
    product_price DECIMAL(10,2) NOT NULL CHECK (product_price >= 0), -- Snapshot of price at time of order
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    total DECIMAL(10,2) NOT NULL CHECK (total >= 0),
    created_at TIMESTAMP DEFAULT NOW()
);

-- Indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_orders_order_number ON orders(order_number);
CREATE INDEX IF NOT EXISTS idx_orders_customer_email ON orders(customer_email);
CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status);
CREATE INDEX IF NOT EXISTS idx_orders_created_at ON orders(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_order_items_order_id ON order_items(order_id);
CREATE INDEX IF NOT EXISTS idx_order_items_product_id ON order_items(product_id);

-- Add comments
COMMENT ON TABLE orders IS 'Stores order information including customer details, addresses, and totals';
COMMENT ON TABLE order_items IS 'Stores individual line items for each order with product snapshots';
COMMENT ON COLUMN orders.order_number IS 'Unique human-readable order identifier (e.g., ORD-2025-001)';
COMMENT ON COLUMN orders.billing_address IS 'JSON object containing billing address fields';
COMMENT ON COLUMN orders.shipping_address IS 'JSON object containing shipping address fields';
COMMENT ON COLUMN orders.status IS 'Order status: pending, processing, completed, cancelled, refunded';
COMMENT ON COLUMN orders.payment_intent_id IS 'Stripe payment intent ID for tracking payments';
COMMENT ON COLUMN order_items.product_name IS 'Product name at time of order (snapshot)';
COMMENT ON COLUMN order_items.product_price IS 'Product price at time of order (snapshot)';

-- Add down migration script here
-- DROP TABLE IF EXISTS order_items;
-- DROP TABLE IF EXISTS orders;
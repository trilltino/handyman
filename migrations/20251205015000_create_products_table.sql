-- Products Table
-- Stores product information for the website sales platform

CREATE TABLE IF NOT EXISTS products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    price DECIMAL(10,2) NOT NULL CHECK (price >= 0),
    category VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Index for faster category filtering
CREATE INDEX IF NOT EXISTS idx_products_category ON products(category);

-- Index for name search
CREATE INDEX IF NOT EXISTS idx_products_name ON products(name);

-- Index for price sorting
CREATE INDEX IF NOT EXISTS idx_products_price ON products(price);

-- Add comment
COMMENT ON TABLE products IS 'Stores product information for the website sales platform';
COMMENT ON COLUMN products.price IS 'Product price in USD';

-- Product Images Table
-- Stores multiple images per product

CREATE TABLE IF NOT EXISTS product_images (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    image_url VARCHAR(500) NOT NULL,
    alt_text VARCHAR(255),
    is_primary BOOLEAN DEFAULT FALSE,
    display_order INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Index for product images lookup
CREATE INDEX IF NOT EXISTS idx_product_images_product_id ON product_images(product_id);

-- Index for primary image lookup
CREATE INDEX IF NOT EXISTS idx_product_images_primary ON product_images(product_id, is_primary);

-- Ensure only one primary image per product
CREATE UNIQUE INDEX IF NOT EXISTS idx_product_images_unique_primary ON product_images(product_id) WHERE is_primary = TRUE;

-- Add comment
COMMENT ON TABLE product_images IS 'Stores product images with ordering and primary image designation';

-- Add down migration script here
-- DROP TABLE IF EXISTS product_images;
-- DROP TABLE IF EXISTS products;
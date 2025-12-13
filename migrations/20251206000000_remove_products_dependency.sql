-- Remove Products Dependency
-- Drops products table and foreign key constraints

-- Drop foreign key constraint on order_items
ALTER TABLE order_items DROP CONSTRAINT IF EXISTS order_items_product_id_fkey;

-- Drop product_images table
DROP TABLE IF EXISTS product_images;

-- Drop products table
DROP TABLE IF EXISTS products;

//! Product data structures for catalog
//!
//! Contains Product and `ProductImage` types for the product marketplace.

use serde::{Deserialize, Serialize};

/// Product for catalog display.
///
/// Contains product information for the marketplace catalog
/// and shopping cart functionality.
///
/// # Fields
///
/// * `id` - Database primary key
/// * `name` - Product name/title
/// * `description` - Optional product description
/// * `price` - Price in USD
/// * `category` - Optional product category
/// * `created_at` - ISO-8601 creation timestamp
/// * `updated_at` - ISO-8601 last update timestamp
///
/// # Example
///
/// ```rust
/// use shared::Product;
///
/// let product = Product::new(1, "Website Package".to_string(), 500.0);
/// assert_eq!(product.id, 1);
/// assert_eq!(product.name, "Website Package");
/// assert_eq!(product.price, 500.0);
/// assert_eq!(product.price_formatted(), "$500.00");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[must_use = "Product should be validated before use"]
pub struct Product {
    /// Auto-generated primary key
    pub id: i32,
    /// Product name
    pub name: String,
    /// Product description (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Product price in USD
    pub price: f64,
    /// Product category (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Timestamp when created (ISO-8601)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Timestamp when last updated (ISO-8601)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl Product {
    /// Creates a new product with minimal fields.
    ///
    /// # Arguments
    ///
    /// * `id` - Product ID (primary key)
    /// * `name` - Product name
    /// * `price` - Product price in USD
    ///
    /// # Example
    ///
    /// ```rust
    /// use shared::Product;
    ///
    /// let product = Product::new(1, "Website".to_string(), 500.0);
    /// assert_eq!(product.id, 1);
    /// ```
    #[must_use]
    pub const fn new(id: i32, name: String, price: f64) -> Self {
        Self {
            id,
            name,
            description: None,
            price,
            category: None,
            created_at: None,
            updated_at: None,
        }
    }

    /// Formats the price as a currency string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shared::Product;
    ///
    /// let product = Product::new(1, "Website".to_string(), 500.0);
    /// assert_eq!(product.price_formatted(), "$500.00");
    /// ```
    #[must_use]
    #[inline]
    pub fn price_formatted(&self) -> String {
        format!("${:.2}", self.price)
    }

    /// Checks if price is valid (non-negative).
    ///
    /// # Example
    ///
    /// ```rust
    /// use shared::Product;
    ///
    /// let product = Product::new(1, "Website".to_string(), 500.0);
    /// assert!(product.is_price_valid());
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_price_valid(&self) -> bool {
        self.price >= 0.0
    }

    /// Validates all product fields.
    ///
    /// Checks that:
    /// - ID is positive
    /// - Name is not empty and not too long
    /// - Price is non-negative and not excessive
    ///
    /// # Returns
    ///
    /// - `Ok(())` if all fields are valid
    /// - `Err(String)` with descriptive error message
    ///
    /// # Example
    ///
    /// ```rust
    /// use shared::Product;
    ///
    /// let valid = Product::new(1, "Website".to_string(), 500.0);
    /// assert!(valid.validate().is_ok());
    ///
    /// let invalid = Product::new(0, "Website".to_string(), -10.0);
    /// assert!(invalid.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        if self.id < 1 {
            return Err("Product ID must be positive".to_string());
        }

        if self.name.is_empty() {
            return Err("Product name is required".to_string());
        }

        if self.name.len() > 255 {
            return Err("Product name must be 255 characters or less".to_string());
        }

        if self.price < 0.0 {
            return Err("Price cannot be negative".to_string());
        }

        if self.price > 999_999.99 {
            return Err("Price exceeds maximum allowed value".to_string());
        }

        Ok(())
    }
}

/// Product image for catalog display.
///
/// Stores image information for products.
/// Supports multiple images per product with ordering.
///
/// # Fields
///
/// * `id` - Database primary key
/// * `product_id` - Foreign key to Product
/// * `image_url` - Image URL (full or relative path)
/// * `alt_text` - Optional alt text for accessibility
/// * `is_primary` - Whether this is the featured image
/// * `display_order` - Display order (lower numbers first)
/// * `created_at` - ISO-8601 creation timestamp
///
/// # Example
///
/// ```rust
/// use shared::ProductImage;
///
/// let image = ProductImage {
///     id: 1,
///     product_id: 1,
///     image_url: "https://example.com/image.jpg".to_string(),
///     alt_text: Some("Product hero view".to_string()),
///     is_primary: true,
///     display_order: 1,
///     created_at: None,
/// };
/// assert!(image.is_featured());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProductImage {
    /// Auto-generated primary key
    pub id: i32,
    /// Product ID this image belongs to
    pub product_id: i32,
    /// Image URL
    pub image_url: String,
    /// Alt text for accessibility (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
    /// Whether this is the primary image
    pub is_primary: bool,
    /// Display order
    pub display_order: i32,
    /// Timestamp when created (ISO-8601)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// Product with generic images list.
///
/// Used for API responses that include product details and associated images.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProductWithImages {
    /// Product data
    #[serde(flatten)]
    pub product: Product,
    /// Product images
    pub images: Vec<ProductImage>,
}

impl ProductImage {
    /// Checks if image is featured/primary.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shared::ProductImage;
    ///
    /// let image = ProductImage {
    ///     id: 1,
    ///     product_id: 1,
    ///     image_url: "image.jpg".to_string(),
    ///     alt_text: None,
    ///     is_primary: true,
    ///     display_order: 1,
    ///     created_at: None,
    /// };
    /// assert!(image.is_featured());
    /// ```
    pub const fn is_featured(&self) -> bool {
        self.is_primary
    }

    /// Validates all product image fields.
    ///
    /// Checks that:
    /// - ID is positive
    /// - Product ID is positive
    /// - Image URL is not empty
    /// - Display order is non-negative
    ///
    /// # Returns
    ///
    /// - `Ok(())` if all fields are valid
    /// - `Err(String)` with descriptive error message
    pub fn validate(&self) -> Result<(), String> {
        if self.id < 1 {
            return Err("Image ID must be positive".to_string());
        }

        if self.product_id < 1 {
            return Err("Product ID must be positive".to_string());
        }

        if self.image_url.is_empty() {
            return Err("Image URL is required".to_string());
        }

        if self.display_order < 0 {
            return Err("Display order must be non-negative".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_new() {
        let product = Product::new(1, "Website".to_string(), 500.0);
        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Website");
        assert_eq!(product.price, 500.0);
        assert_eq!(product.description, None);
    }

    #[test]
    fn test_product_price_formatted() {
        let product = Product::new(1, "Website".to_string(), 500.0);
        assert_eq!(product.price_formatted(), "$500.00");

        let product2 = Product::new(2, "Basic".to_string(), 99.99);
        assert_eq!(product2.price_formatted(), "$99.99");
    }

    #[test]
    fn test_product_is_price_valid() {
        let valid = Product::new(1, "Website".to_string(), 500.0);
        assert!(valid.is_price_valid());

        let invalid = Product::new(2, "Free".to_string(), -10.0);
        assert!(!invalid.is_price_valid());

        let zero = Product::new(3, "Promo".to_string(), 0.0);
        assert!(zero.is_price_valid());
    }

    #[test]
    fn test_product_serialization() {
        let product = Product::new(1, "Website".to_string(), 500.0);
        let json = serde_json::to_string(&product).unwrap();
        let deserialized: Product = serde_json::from_str(&json).unwrap();
        assert_eq!(product, deserialized);
    }

    #[test]
    fn test_product_validate_valid() {
        let product = Product::new(1, "Website".to_string(), 500.0);
        assert!(product.validate().is_ok());
    }

    #[test]
    fn test_product_validate_invalid_id() {
        let product = Product::new(0, "Website".to_string(), 500.0);
        assert!(product.validate().is_err());
        assert!(product.validate().unwrap_err().contains("ID"));
    }

    #[test]
    fn test_product_validate_negative_id() {
        let product = Product::new(-1, "Website".to_string(), 500.0);
        assert!(product.validate().is_err());
    }

    #[test]
    fn test_product_validate_empty_name() {
        let product = Product {
            id: 1,
            name: "".to_string(),
            description: None,
            price: 500.0,
            category: None,
            created_at: None,
            updated_at: None,
        };
        assert!(product.validate().is_err());
        assert!(product.validate().unwrap_err().contains("name"));
    }

    #[test]
    fn test_product_validate_name_too_long() {
        let product = Product {
            id: 1,
            name: "a".repeat(256),
            description: None,
            price: 500.0,
            category: None,
            created_at: None,
            updated_at: None,
        };
        assert!(product.validate().is_err());
    }

    #[test]
    fn test_product_validate_negative_price() {
        let product = Product::new(1, "Website".to_string(), -10.0);
        assert!(product.validate().is_err());
        assert!(product.validate().unwrap_err().contains("negative"));
    }

    #[test]
    fn test_product_validate_excessive_price() {
        let product = Product::new(1, "Website".to_string(), 1_000_000.0);
        assert!(product.validate().is_err());
    }

    #[test]
    fn test_product_image_is_featured() {
        let featured = ProductImage {
            id: 1,
            product_id: 1,
            image_url: "image.jpg".to_string(),
            alt_text: None,
            is_primary: true,
            display_order: 1,
            created_at: None,
        };
        assert!(featured.is_featured());

        let not_featured = ProductImage {
            id: 2,
            product_id: 1,
            image_url: "image2.jpg".to_string(),
            alt_text: None,
            is_primary: false,
            display_order: 2,
            created_at: None,
        };
        assert!(!not_featured.is_featured());
    }

    #[test]
    fn test_product_image_serialization() {
        let image = ProductImage {
            id: 1,
            product_id: 1,
            image_url: "image.jpg".to_string(),
            alt_text: Some("Hero".to_string()),
            is_primary: true,
            display_order: 1,
            created_at: None,
        };

        let json = serde_json::to_string(&image).unwrap();
        let deserialized: ProductImage = serde_json::from_str(&json).unwrap();
        assert_eq!(image, deserialized);
    }

    #[test]
    fn test_product_image_validate_valid() {
        let image = ProductImage {
            id: 1,
            product_id: 1,
            image_url: "image.jpg".to_string(),
            alt_text: None,
            is_primary: true,
            display_order: 1,
            created_at: None,
        };
        assert!(image.validate().is_ok());
    }

    #[test]
    fn test_product_image_validate_invalid_id() {
        let image = ProductImage {
            id: 0,
            product_id: 1,
            image_url: "image.jpg".to_string(),
            alt_text: None,
            is_primary: true,
            display_order: 1,
            created_at: None,
        };
        assert!(image.validate().is_err());
        assert!(image.validate().unwrap_err().contains("Image ID"));
    }

    #[test]
    fn test_product_image_validate_invalid_product_id() {
        let image = ProductImage {
            id: 1,
            product_id: 0,
            image_url: "image.jpg".to_string(),
            alt_text: None,
            is_primary: true,
            display_order: 1,
            created_at: None,
        };
        assert!(image.validate().is_err());
        assert!(image.validate().unwrap_err().contains("Product ID"));
    }

    #[test]
    fn test_product_image_validate_empty_url() {
        let image = ProductImage {
            id: 1,
            product_id: 1,
            image_url: "".to_string(),
            alt_text: None,
            is_primary: true,
            display_order: 1,
            created_at: None,
        };
        assert!(image.validate().is_err());
        assert!(image.validate().unwrap_err().contains("URL"));
    }

    #[test]
    fn test_product_image_validate_negative_display_order() {
        let image = ProductImage {
            id: 1,
            product_id: 1,
            image_url: "image.jpg".to_string(),
            alt_text: None,
            is_primary: true,
            display_order: -1,
            created_at: None,
        };
        assert!(image.validate().is_err());
        assert!(image.validate().unwrap_err().contains("Display order"));
    }
}

//! Blog post structured data schema

/// Creates `BlogPosting` structured data for individual blog posts.
///
/// This schema provides search engines with metadata about blog posts,
/// enabling rich search results with article information.
///
/// # Arguments
///
/// * `title` - Blog post title
/// * `description` - Blog post summary/excerpt
/// * `url` - Full canonical URL of the blog post
/// * `image` - Featured image URL
///
/// # Returns
///
/// A JSON-LD `BlogPosting` schema object.
#[must_use]
pub fn create_blog_post_schema(
    title: String,
    description: String,
    url: String,
    image: String,
) -> serde_json::Value {
    serde_json::json!({
        "@context": "https://schema.org",
        "@type": "BlogPosting",
        "headline": title,
        "description": description,
        "image": image,
        "url": url,
        "author": {
            "@type": "Organization",
            "name": "Handyman Marketplace"
        },
        "publisher": {
            "@type": "Organization",
            "name": "Handyman Marketplace",
            "logo": {
                "@type": "ImageObject",
                "url": "https://handymanmarketplace.com/logo.png"
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blog_post_schema_has_required_fields() {
        let schema = create_blog_post_schema(
            "Test Post".to_string(),
            "Test description".to_string(),
            "https://example.com/post".to_string(),
            "https://example.com/image.jpg".to_string(),
        );

        assert_eq!(schema["@context"], "https://schema.org");
        assert_eq!(schema["@type"], "BlogPosting");
        assert_eq!(schema["headline"], "Test Post");
        assert_eq!(schema["description"], "Test description");
    }

    #[test]
    fn test_blog_post_schema_has_author_and_publisher() {
        let schema = create_blog_post_schema(
            "Test Post".to_string(),
            "Test description".to_string(),
            "https://example.com/post".to_string(),
            "https://example.com/image.jpg".to_string(),
        );

        assert_eq!(schema["author"]["@type"], "Organization");
        assert_eq!(schema["author"]["name"], "Handyman Marketplace");
        assert_eq!(schema["publisher"]["@type"], "Organization");
        assert!(schema["publisher"]["logo"]["@type"].is_string());
    }

    #[test]
    fn test_blog_post_schema_serializable() {
        let schema = create_blog_post_schema(
            "Test Post".to_string(),
            "Test description".to_string(),
            "https://example.com/post".to_string(),
            "https://example.com/image.jpg".to_string(),
        );

        let json = serde_json::to_string(&schema).unwrap();
        let deserialized: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(schema, deserialized);
    }

    #[test]
    fn test_blog_post_schema_with_custom_values() {
        let title = "How to Market Your Handyman Business";
        let description = "Learn the best strategies...";
        let url = "https://handymanmarketplace.com/blog/marketing-tips";
        let image = "https://handymanmarketplace.com/blog/marketing.jpg";

        let schema = create_blog_post_schema(
            title.to_string(),
            description.to_string(),
            url.to_string(),
            image.to_string(),
        );

        assert_eq!(schema["headline"], title);
        assert_eq!(schema["description"], description);
        assert_eq!(schema["url"], url);
        assert_eq!(schema["image"], image);
    }
}

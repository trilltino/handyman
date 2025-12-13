//! Service structured data schema

/// Creates Service structured data for website building services.
///
/// This schema describes the website building service offered, including
/// pricing information and service area.
///
/// # Returns
///
/// A JSON-LD `Service` schema object.
#[must_use]
pub fn create_service_schema() -> serde_json::Value {
    serde_json::json!({
        "@context": "https://schema.org",
        "@type": "Service",
        "name": "Professional Handyman Website Design",
        "description": "Complete website building service for handymen and contractors including design, development, booking systems, and marketing tools",
        "provider": {
            "@type": "Organization",
            "name": "Handyman Marketplace"
        },
        "areaServed": "GB",
        "serviceType": "Website Development",
        "offers": {
            "@type": "Offer",
            "priceRange": "£500-£2000",
            "description": "Professional handyman website with booking system"
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_schema_has_required_fields() {
        let schema = create_service_schema();
        assert_eq!(schema["@context"], "https://schema.org");
        assert_eq!(schema["@type"], "Service");
        assert!(schema["name"].is_string());
        assert!(schema["description"].is_string());
    }

    #[test]
    fn test_service_schema_has_provider() {
        let schema = create_service_schema();
        assert_eq!(schema["provider"]["@type"], "Organization");
        assert_eq!(schema["provider"]["name"], "Handyman Marketplace");
    }

    #[test]
    fn test_service_schema_has_offers() {
        let schema = create_service_schema();
        assert_eq!(schema["offers"]["@type"], "Offer");
        assert!(schema["offers"]["priceRange"].is_string());
    }

    #[test]
    fn test_service_schema_serializable() {
        let schema = create_service_schema();
        let json = serde_json::to_string(&schema).unwrap();
        let deserialized: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(schema, deserialized);
    }
}

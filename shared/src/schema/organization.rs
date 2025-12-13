//! Organization structured data schema

/// Creates Organization structured data for Handyman Marketplace.
///
/// This schema provides search engines with information about the business,
/// including contact details, service area, and areas of expertise.
///
/// # Returns
///
/// A JSON-LD `Organization` schema object.
///
/// # Example Output
///
/// ```json
/// {
///   "@context": "https://schema.org",
///   "@type": "Organization",
///   "name": "Handyman Marketplace",
///   "url": "https://handymanmarketplace.com",
///   ...
/// }
/// ```
#[must_use]
pub fn create_organization_schema() -> serde_json::Value {
    serde_json::json!({
        "@context": "https://schema.org",
        "@type": "Organization",
        "name": "Handyman Marketplace",
        "description": "Professional website building service specializing in handyman and contractor websites",
        "url": "https://handymanmarketplace.com",
        "logo": "https://handymanmarketplace.com/logo.png",
        "contactPoint": {
            "@type": "ContactPoint",
            "telephone": "+44-123-456-7890",
            "contactType": "customer service",
            "availableLanguage": "English"
        },
        "areaServed": {
            "@type": "Country",
            "name": "United Kingdom"
        },
        "serviceType": ["Website Design", "Web Development", "Digital Marketing"],
        "knowsAbout": [
            "Handyman Websites",
            "Contractor Marketing",
            "Professional Web Design",
            "Lead Generation",
            "Online Booking Systems"
        ]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organization_schema_has_required_fields() {
        let schema = create_organization_schema();
        assert_eq!(schema["@context"], "https://schema.org");
        assert_eq!(schema["@type"], "Organization");
        assert_eq!(schema["name"], "Handyman Marketplace");
        assert!(schema["url"].is_string());
    }

    #[test]
    fn test_organization_schema_has_contact_point() {
        let schema = create_organization_schema();
        assert!(schema["contactPoint"]["@type"].is_string());
        assert!(schema["contactPoint"]["telephone"].is_string());
    }

    #[test]
    fn test_organization_schema_serializable() {
        let schema = create_organization_schema();
        let json = serde_json::to_string(&schema).unwrap();
        let deserialized: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(schema, deserialized);
    }
}

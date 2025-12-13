//! FAQ structured data schema

/// Creates FAQ structured data for common questions.
///
/// This schema enables rich FAQ results in search engines, potentially
/// displaying questions and answers directly in search results.
///
/// # Returns
///
/// A JSON-LD `FAQPage` schema object with common handyman website questions.
#[must_use]
pub fn create_faq_schema() -> serde_json::Value {
    serde_json::json!({
        "@context": "https://schema.org",
        "@type": "FAQPage",
        "mainEntity": [
            {
                "@type": "Question",
                "name": "How long does it take to build a handyman website?",
                "acceptedAnswer": {
                    "@type": "Answer",
                    "text": "We deliver professional handyman websites in just 7 days from order to launch. This includes custom design, booking system integration, and full testing."
                }
            },
            {
                "@type": "Question",
                "name": "Do I need technical skills to manage my website?",
                "acceptedAnswer": {
                    "@type": "Answer",
                    "text": "No technical skills required. We handle everything including hosting, updates, security, and ongoing support. You focus on your handyman business."
                }
            },
            {
                "@type": "Question",
                "name": "What does a professional handyman website include?",
                "acceptedAnswer": {
                    "@type": "Answer",
                    "text": "Professional handyman websites include custom design, online booking system, service listings, customer testimonials, contact forms, and SEO optimization."
                }
            },
            {
                "@type": "Question",
                "name": "How much does a handyman website cost?",
                "acceptedAnswer": {
                    "@type": "Answer",
                    "text": "Handyman websites start from £500 for our basic package, including everything you need to get online and start accepting bookings."
                }
            },
            {
                "@type": "Question",
                "name": "Do you provide ongoing support after launch?",
                "acceptedAnswer": {
                    "@type": "Answer",
                    "text": "Yes, all packages include 12 months of free support, updates, security monitoring, and email support to ensure your website continues working perfectly."
                }
            }
        ]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_faq_schema_has_required_fields() {
        let schema = create_faq_schema();
        assert_eq!(schema["@context"], "https://schema.org");
        assert_eq!(schema["@type"], "FAQPage");
        assert!(schema["mainEntity"].is_array());
    }

    #[test]
    fn test_faq_schema_has_questions() {
        let schema = create_faq_schema();
        let main_entity = schema["mainEntity"].as_array().unwrap();
        assert!(main_entity.len() >= 5);

        for question in main_entity {
            assert_eq!(question["@type"], "Question");
            assert!(question["name"].is_string());
            assert_eq!(question["acceptedAnswer"]["@type"], "Answer");
            assert!(question["acceptedAnswer"]["text"].is_string());
        }
    }

    #[test]
    fn test_faq_schema_serializable() {
        let schema = create_faq_schema();
        let json = serde_json::to_string(&schema).unwrap();
        let deserialized: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(schema, deserialized);
    }
}

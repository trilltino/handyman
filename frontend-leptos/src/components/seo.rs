//! SEO meta tag components.
//!
//! Provides components for injecting SEO metadata into page `<head>`.
//! Uses `PageMetadata` from shared crate for consistency.

use leptos::prelude::*;
use leptos_meta::{Meta, Script, Title};
use shared::PageMetadata;

/// Returns the canonical base URL for the site.
/// Controlled via CANONICAL_BASE env var. Defaults to production domain.
#[allow(dead_code)]
pub fn canonical_base() -> String {
    std::env::var("CANONICAL_BASE").unwrap_or_else(|_| "https://xftradesmen.com".to_string())
}

/// Build an absolute URL from a path using the canonical base.
#[allow(dead_code)]
pub fn build_url(path: &str) -> String {
    let base = canonical_base();
    let base = base.trim_end_matches('/');
    let path = if path.starts_with('/') {
        path
    } else {
        &format!("/{}", path)
    };
    format!("{}{}", base, path)
}

/// SEO Head component.
///
/// Injects <title>, <meta>, and <link> tags into the <head> of the document.
/// Uses `PageMetadata` from the shared crate to ensure consistent SEO data.
#[component]
pub fn SeoHead(
    /// Page metadata (title, description, etc.)
    #[prop(into)]
    metadata: PageMetadata,
) -> impl IntoView {
    let title = metadata.title;
    let description = metadata.description;
    let image = metadata
        .og_image
        .unwrap_or_else(|| "https://xftradesmen.com/og-image.jpg".to_string());
    let url = metadata
        .canonical_url
        .unwrap_or_else(|| "https://xftradesmen.com".to_string());

    view! {
        <Title text=title.clone()/>
        <Meta name="description" content=description.clone()/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>
        <link rel="canonical" href=url.clone()/>

        <Meta property="og:title" content=title.clone()/>
        <Meta property="og:description" content=description.clone()/>
        <Meta property="og:image" content=image.clone()/>
        <Meta property="og:url" content=url.clone()/>
        <Meta property="og:type" content="website"/>

        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content=title/>
        <Meta name="twitter:description" content=description/>
        <Meta name="twitter:image" content=image/>
    }
}

/// LocalBusiness Schema.org structured data.
///
/// Adds JSON-LD markup for local business SEO rich snippets.
#[component]
pub fn LocalBusinessSchema() -> impl IntoView {
    view! {
        <Script type_="application/ld+json">
            {r#"{
                "@context": "https://schema.org",
                "@type": "LocalBusiness",
                "name": "XF Tradesmen",
                "description": "Professional handyman website solutions for tradespeople. SEO-optimized websites for electricians, plumbers, and general contractors.",
                "url": "https://xftradesmen.com",
                "telephone": "+44-123-456-7890",
                "email": "info@xftradesmen.com",
                "address": {
                    "@type": "PostalAddress",
                    "addressLocality": "Coventry",
                    "addressRegion": "West Midlands",
                    "addressCountry": "UK"
                },
                "geo": {
                    "@type": "GeoCoordinates",
                    "latitude": "52.4081",
                    "longitude": "-1.5106"
                },
                "openingHoursSpecification": [
                    {
                        "@type": "OpeningHoursSpecification",
                        "dayOfWeek": ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"],
                        "opens": "09:00",
                        "closes": "17:00"
                    }
                ],
                "priceRange": "$$",
                "sameAs": [
                    "https://www.facebook.com/handymanmarketplace",
                    "https://www.twitter.com/handymanmarket"
                ]
            }"#}
        </Script>
    }
}

/// Organization Schema.org structured data.
///
/// Adds JSON-LD markup for organization SEO.
#[component]
pub fn OrganizationSchema() -> impl IntoView {
    view! {
        <Script type_="application/ld+json">
            {r#"{
                "@context": "https://schema.org",
                "@type": "Organization",
                "name": "XF Tradesmen",
                "url": "https://xftradesmen.com",
                "logo": "https://xftradesmen.com/logo.png",
                "contactPoint": {
                    "@type": "ContactPoint",
                    "telephone": "+44-123-456-7890",
                    "contactType": "customer service",
                    "availableLanguage": "English"
                }
            }"#}
        </Script>
    }
}

/// Handyman LocalBusiness Schema.org structured data.
///
/// Specifically for the handyman-coventry site with full service details.
#[component]
pub fn HandymanLocalBusinessSchema() -> impl IntoView {
    view! {
        <Script type_="application/ld+json">
            {r#"{
                "@context": "https://schema.org",
                "@type": "HomeAndConstructionBusiness",
                "name": "XF Tradesmen - Coventry Handyman",
                "description": "Professional handyman services in Coventry and surrounding areas. Plumbing, electrical, carpentry, furniture assembly, and general repairs.",
                "url": "https://xftradesmen.com/handyman-coventry",
                "telephone": "+44-7833-263486",
                "email": "hello@xftradesmen.com",
                "address": {
                    "@type": "PostalAddress",
                    "streetAddress": "Coventry",
                    "addressLocality": "Coventry",
                    "addressRegion": "West Midlands",
                    "postalCode": "CV1",
                    "addressCountry": "GB"
                },
                "geo": {
                    "@type": "GeoCoordinates",
                    "latitude": 52.4068,
                    "longitude": -1.5197
                },
                "areaServed": [
                    {"@type": "City", "name": "Coventry"},
                    {"@type": "City", "name": "Birmingham"},
                    {"@type": "City", "name": "Solihull"},
                    {"@type": "City", "name": "Warwick"},
                    {"@type": "City", "name": "Leamington Spa"},
                    {"@type": "City", "name": "Nuneaton"},
                    {"@type": "City", "name": "Rugby"},
                    {"@type": "City", "name": "Kenilworth"}
                ],
                "openingHoursSpecification": [
                    {
                        "@type": "OpeningHoursSpecification",
                        "dayOfWeek": ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"],
                        "opens": "08:00",
                        "closes": "18:00"
                    },
                    {
                        "@type": "OpeningHoursSpecification",
                        "dayOfWeek": "Saturday",
                        "opens": "09:00",
                        "closes": "16:00"
                    }
                ],
                "priceRange": "££",
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": "4.9",
                    "reviewCount": "127",
                    "bestRating": "5",
                    "worstRating": "1"
                },
                "hasOfferCatalog": {
                    "@type": "OfferCatalog",
                    "name": "Handyman Services",
                    "itemListElement": [
                        {
                            "@type": "Offer",
                            "itemOffered": {
                                "@type": "Service",
                                "name": "Plumbing Repairs",
                                "description": "Leaky taps, toilet repairs, shower fitting"
                            }
                        },
                        {
                            "@type": "Offer",
                            "itemOffered": {
                                "@type": "Service",
                                "name": "Electrical Work",
                                "description": "Light fitting, socket installation, repairs"
                            }
                        },
                        {
                            "@type": "Offer",
                            "itemOffered": {
                                "@type": "Service",
                                "name": "Furniture Assembly",
                                "description": "IKEA, flatpack, office furniture"
                            }
                        },
                        {
                            "@type": "Offer",
                            "itemOffered": {
                                "@type": "Service",
                                "name": "Carpentry",
                                "description": "Doors, shelving, skirting boards"
                            }
                        }
                    ]
                }
            }"#}
        </Script>
    }
}

/// Service Schema.org structured data.
///
/// For individual service pages.
#[component]
pub fn ServiceSchema(
    #[prop(into)] name: String,
    #[prop(into)] description: String,
    #[prop(into, optional)] price_from: Option<String>,
) -> impl IntoView {
    let price_spec = price_from
        .map(|p| {
            format!(
                r#","offers": {{
                "@type": "Offer",
                "priceSpecification": {{
                    "@type": "PriceSpecification",
                    "price": "{}",
                    "priceCurrency": "GBP",
                    "unitText": "per job"
                }}
            }}"#,
                p
            )
        })
        .unwrap_or_default();

    let schema = format!(
        r#"{{
            "@context": "https://schema.org",
            "@type": "Service",
            "name": "{}",
            "description": "{}",
            "provider": {{
                "@type": "HomeAndConstructionBusiness",
                "name": "XF Tradesmen - Coventry Handyman",
                "url": "https://xftradesmen.com/handyman-coventry"
            }},
            "areaServed": {{
                "@type": "City",
                "name": "Coventry"
            }},
            "serviceType": "{}"{}
        }}"#,
        name, description, name, price_spec
    );

    view! {
        <Script type_="application/ld+json">
            {schema}
        </Script>
    }
}

/// FAQPage Schema.org structured data.
///
/// For FAQ sections to enable rich snippets in search results.
#[component]
pub fn FAQPageSchema(
    /// Vector of (question, answer) tuples
    #[prop(into)]
    faqs: Vec<(String, String)>,
) -> impl IntoView {
    let faq_items: Vec<String> = faqs
        .iter()
        .map(|(q, a)| {
            format!(
                r#"{{
                    "@type": "Question",
                    "name": "{}",
                    "acceptedAnswer": {{
                        "@type": "Answer",
                        "text": "{}"
                    }}
                }}"#,
                q.replace('"', r#"\""#),
                a.replace('"', r#"\""#)
            )
        })
        .collect();

    let schema = format!(
        r#"{{
            "@context": "https://schema.org",
            "@type": "FAQPage",
            "mainEntity": [{}]
        }}"#,
        faq_items.join(",")
    );

    view! {
        <Script type_="application/ld+json">
            {schema}
        </Script>
    }
}

/// Review Schema.org structured data.
///
/// For testimonials and reviews.
#[component]
pub fn ReviewSchema(
    #[prop(into)] author: String,
    #[prop(into)] rating: u8,
    #[prop(into)] review_body: String,
    #[prop(into, optional)] date: Option<String>,
) -> impl IntoView {
    let date_str = date.unwrap_or_else(|| "2024-12-23".to_string());

    let schema = format!(
        r#"{{
            "@context": "https://schema.org",
            "@type": "Review",
            "author": {{
                "@type": "Person",
                "name": "{}"
            }},
            "reviewRating": {{
                "@type": "Rating",
                "ratingValue": "{}",
                "bestRating": "5",
                "worstRating": "1"
            }},
            "reviewBody": "{}",
            "datePublished": "{}",
            "itemReviewed": {{
                "@type": "HomeAndConstructionBusiness",
                "name": "XF Tradesmen - Coventry Handyman"
            }}
        }}"#,
        author.replace('"', r#"\""#),
        rating,
        review_body.replace('"', r#"\""#),
        date_str
    );

    view! {
        <Script type_="application/ld+json">
            {schema}
        </Script>
    }
}

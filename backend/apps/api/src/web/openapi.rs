//! OpenAPI documentation configuration.
//!
//! Provides Swagger UI and OpenAPI spec generation.

use lib_core::model::contact::ContactForCreate;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::web::handlers::contact::api_contact_handler,
        crate::web::handlers::static_content::health_handler,
        crate::web::handlers::static_content::version_handler,
        crate::web::handlers::static_content::stripe_config_handler
    ),
    components(
        schemas(
            ContactForCreate
        )
    ),
    tags(
        (name = "contact", description = "Contact form endpoints"),
        (name = "payments", description = "Payment and Stripe endpoints"),
        (name = "health", description = "Health check endpoints")
    )
)]
#[allow(dead_code)]
pub struct ApiDoc;

#[allow(dead_code)]
pub fn swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi())
}

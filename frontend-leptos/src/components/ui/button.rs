//! Button component for navigation links.
//!
//! Provides styled link buttons with variants (Primary, Secondary, Ghost).

use leptos::prelude::*;

/// Button style variants.
///
/// Defines the visual style of buttons throughout the application.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum ButtonVariant {
    /// Primary action button - Red Gradient
    Primary,
    /// Secondary action button - Dark Surface
    Secondary,
    /// Transparent ghost button - Red text on hover
    Ghost,
}

impl ButtonVariant {
    /// Get Tailwind CSS class for this variant.
    pub fn class(&self) -> &str {
        match self {
            ButtonVariant::Primary => "btn-primary",
            ButtonVariant::Secondary => "btn-secondary",
            ButtonVariant::Ghost => "btn-ghost",
        }
    }
}

/// Styled link button component.
#[component]
pub fn Button(
    /// Button style variant
    #[prop(default = ButtonVariant::Primary)]
    variant: ButtonVariant,
    /// Link destination URL
    #[prop(into)]
    href: String,
    /// Open in new tab if true
    #[prop(default = false)]
    external: bool,
    /// Button text
    #[prop(into)]
    label: String,
) -> impl IntoView {
    let variant_class = variant.class();
    let rel = if external { "noopener noreferrer" } else { "" };
    let target = if external { "_blank" } else { "" };

    view! {
        <a href=href class=format!("btn {}", variant_class) rel=rel target=target>
            {label}
        </a>
    }
}

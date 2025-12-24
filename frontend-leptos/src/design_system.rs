//! # Design System Constants
//!
//! Centralized design tokens for consistent styling across the application.
//!
//! ## Usage
//!
//! ```rust
//! use crate::design_system::*;
//!
//! let color = colors::PRIMARY;
//! let space = spacing::MD;
//! ```

/// Color palette - Tailwind-compatible hex values
pub mod colors {
    // Primary brand colors
    pub const PRIMARY: &str = "#2563eb"; // Blue 600
    pub const PRIMARY_DARK: &str = "#1d4ed8"; // Blue 700
    pub const PRIMARY_LIGHT: &str = "#3b82f6"; // Blue 500

    // Secondary colors
    pub const SECONDARY: &str = "#8b5cf6"; // Violet 500
    pub const SECONDARY_DARK: &str = "#7c3aed"; // Violet 600

    // Semantic colors
    pub const SUCCESS: &str = "#22c55e"; // Green 500
    pub const WARNING: &str = "#f59e0b"; // Amber 500
    pub const ERROR: &str = "#ef4444"; // Red 500
    pub const INFO: &str = "#06b6d4"; // Cyan 500

    // Neutral colors
    pub const WHITE: &str = "#ffffff";
    pub const BLACK: &str = "#000000";
    pub const GRAY_50: &str = "#f9fafb";
    pub const GRAY_100: &str = "#f3f4f6";
    pub const GRAY_200: &str = "#e5e7eb";
    pub const GRAY_300: &str = "#d1d5db";
    pub const GRAY_400: &str = "#9ca3af";
    pub const GRAY_500: &str = "#6b7280";
    pub const GRAY_600: &str = "#4b5563";
    pub const GRAY_700: &str = "#374151";
    pub const GRAY_800: &str = "#1f2937";
    pub const GRAY_900: &str = "#111827";
}

/// Spacing scale (in Tailwind units, 1 = 0.25rem = 4px)
pub mod spacing {
    pub const XS: &str = "1"; // 4px
    pub const SM: &str = "2"; // 8px
    pub const MD: &str = "4"; // 16px
    pub const LG: &str = "6"; // 24px
    pub const XL: &str = "8"; // 32px
    pub const XXL: &str = "12"; // 48px
    pub const XXXL: &str = "16"; // 64px
}

/// Border radius values
pub mod radius {
    pub const NONE: &str = "rounded-none";
    pub const SM: &str = "rounded-sm";
    pub const DEFAULT: &str = "rounded";
    pub const MD: &str = "rounded-md";
    pub const LG: &str = "rounded-lg";
    pub const XL: &str = "rounded-xl";
    pub const XXL: &str = "rounded-2xl";
    pub const FULL: &str = "rounded-full";
}

/// Shadow presets
pub mod shadows {
    pub const SM: &str = "shadow-sm";
    pub const DEFAULT: &str = "shadow";
    pub const MD: &str = "shadow-md";
    pub const LG: &str = "shadow-lg";
    pub const XL: &str = "shadow-xl";
    pub const XXL: &str = "shadow-2xl";
    pub const INNER: &str = "shadow-inner";
    pub const NONE: &str = "shadow-none";
}

/// Typography presets
pub mod typography {
    // Font families
    pub const FONT_SANS: &str = "font-sans"; // Inter
    pub const FONT_DISPLAY: &str = "font-display"; // Outfit

    // Font sizes
    pub const TEXT_XS: &str = "text-xs"; // 12px
    pub const TEXT_SM: &str = "text-sm"; // 14px
    pub const TEXT_BASE: &str = "text-base"; // 16px
    pub const TEXT_LG: &str = "text-lg"; // 18px
    pub const TEXT_XL: &str = "text-xl"; // 20px
    pub const TEXT_2XL: &str = "text-2xl"; // 24px
    pub const TEXT_3XL: &str = "text-3xl"; // 30px
    pub const TEXT_4XL: &str = "text-4xl"; // 36px

    // Font weights
    pub const FONT_NORMAL: &str = "font-normal";
    pub const FONT_MEDIUM: &str = "font-medium";
    pub const FONT_SEMIBOLD: &str = "font-semibold";
    pub const FONT_BOLD: &str = "font-bold";
}

/// Animation presets
pub mod animations {
    pub const TRANSITION_FAST: &str = "transition duration-150";
    pub const TRANSITION_DEFAULT: &str = "transition duration-300";
    pub const TRANSITION_SLOW: &str = "transition duration-500";
    pub const HOVER_SCALE: &str = "hover:scale-105";
    pub const HOVER_LIFT: &str = "hover:-translate-y-1";
}

/// Common component class combinations
pub mod components {
    pub const CARD: &str = "bg-white rounded-lg shadow-md p-6";
    pub const CARD_HOVER: &str = "bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition";
    pub const BUTTON_PRIMARY: &str =
        "bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 transition";
    pub const BUTTON_SECONDARY: &str =
        "bg-gray-200 text-gray-800 px-4 py-2 rounded-lg hover:bg-gray-300 transition";
    pub const INPUT: &str =
        "w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent";
}

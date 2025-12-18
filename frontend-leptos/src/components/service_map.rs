//! Service map component using Leaflet.
//!
//! Displays an interactive map showing service coverage area.

use leptos::prelude::*;
use leptos_leaflet::prelude::{Circle, MapContainer, Marker, Position, TileLayer, Tooltip};

/// Interactive map showing service coverage area.
///
/// Uses OpenStreetMap tiles with a circle overlay to show the service radius.
#[component]
pub fn ServiceMap(
    /// Latitude (default: Coventry city center)
    #[prop(default = 52.4081)]
    lat: f64,
    /// Longitude (default: Coventry city center)
    #[prop(default = -1.5106)]
    lng: f64,
    /// Zoom level (default: 11 for city-wide view)
    #[prop(default = 11.0)]
    zoom: f64,
    /// Service radius in meters (default: 15km)
    #[prop(default = 15000.0)]
    radius: f64,
    /// Tooltip label for the marker
    #[prop(default = "Service Centre")]
    label: &'static str,
    /// Circle color
    #[prop(default = "#DC2626")]
    color: &'static str,
) -> impl IntoView {
    view! {
        <div class="h-[400px] w-full rounded-xl overflow-hidden shadow-lg border border-void-highlight">
            <MapContainer
                style="height: 100%; width: 100%"
                center=Position::new(lat, lng)
                zoom=zoom
                scroll_wheel_zoom=false
            >
                // OpenStreetMap Dark Tiles (CartoDB Dark Matter)
                <TileLayer
                    url="https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png"
                    attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors &copy; <a href=\"https://carto.com/attributions\">CARTO</a>"
                />

                // Service Area Circle
                <Circle
                    center=Position::new(lat, lng)
                    radius=radius
                    color=color
                    fill_color=color
                    fill_opacity=0.15
                />

                // Main Location Marker
                <Marker position=Position::new(lat, lng)>
                    <Tooltip permanent=true direction="top">
                        {label}
                    </Tooltip>
                </Marker>
            </MapContainer>
        </div>
    }
}

/// Coventry-specific service map with pre-configured coordinates.
#[component]
pub fn CoventryServiceMap() -> impl IntoView {
    view! {
        <ServiceMap
            lat=52.4081
            lng=-1.5106
            zoom=11.0
            radius=20000.0
            label="Coventry & Warwickshire"
        />
    }
}

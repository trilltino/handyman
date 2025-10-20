//! # Map Page (Leaflet Backup)
//!
//! ## Purpose
//! Alternative map implementation using Leaflet.js instead of Cesium.
//! This is a backup/reference implementation kept for comparison purposes.
//!
//! ## How It Works
//! 1. **Leaflet Integration**: Uses Leaflet.js library for 2D interactive maps
//! 2. **OpenStreetMap Tiles**: Free map tiles from OpenStreetMap (no API key needed)
//! 3. **Dynamic Script Injection**: Creates and injects JavaScript to initialize map
//! 4. **Service Markers**: Displays draggable markers for handyman service locations
//! 5. **UI Controls**: Style switcher, quick jump, marker management
//!
//! ## Features
//! - Multiple map styles (street, satellite, terrain, dark mode)
//! - Draggable service location markers
//! - Quick navigation to Portugal cities (Lisbon, Porto, Faro)
//! - Programmatic API (addServiceMarker, flyToLocation, changeMapStyle)
//! - Browser console integration for debugging
//!
//! ## Relation to Entire Program
//! - **Alternative Implementation**: Not currently used, replaced by Cesium 3D map
//! - **Reference Code**: Kept as backup if 2D Leaflet map is preferred over Cesium
//! - **Comparison**: Leaflet (2D, lighter) vs Cesium (3D, more features)
//! - **Route**: Would map to /map route in routes.rs
//!
//! ## Differences from Current Map (map.rs)
//! - **Leaflet vs Cesium**: 2D map library vs 3D globe visualization
//! - **Performance**: Leaflet is lighter, Cesium offers 3D capabilities
//! - **Use Case**: Leaflet for simple location display, Cesium for immersive experience

use yew::prelude::*;  // Yew framework for UI components
use stylist::css;     // CSS-in-Rust styling
use wasm_bindgen::prelude::*;  // JavaScript interop bindings
use web_sys::HtmlElement;       // DOM element types
use gloo::timers::callback::Timeout;  // Async timer for delayed execution

// WASM binding to JavaScript console.log for debugging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);  // Bridge to browser console
}

/// MapPage component - Interactive Leaflet map centered on Portugal
///
/// Displays a 2D map using Leaflet.js with OpenStreetMap tiles.
/// Includes service location markers, map style switcher, and navigation controls.
#[function_component]
pub fn MapPage() -> Html {
    // Reference to map container div element
    let map_container_ref = use_node_ref();

    {
        let map_container_ref = map_container_ref.clone();

        // Effect hook: Initialize Leaflet map after component mounts
        // Delayed by 300ms to ensure DOM is fully ready
        use_effect_with((), move |_| {
            let timeout = Timeout::new(300, move || {
                if let Some(_container) = map_container_ref.cast::<HtmlElement>() {
                    // Create a script element to inject Leaflet initialization code
                    let script = web_sys::window()
                        .and_then(|w| w.document())
                        .and_then(|doc| doc.create_element("script").ok())
                        .and_then(|s| s.dyn_into::<web_sys::HtmlScriptElement>().ok());

                    if let Some(script_elem) = script {
                        // Inject JavaScript code to initialize Leaflet map
                        // This creates the map, adds tiles, markers, and UI controls
                        script_elem.set_text_content(Some(r#"
                            (function() {
                                // Initialize Leaflet map centered on Portugal
                                const map = L.map('map-container', {
                                    center: [38.7223, -9.1393], // Lisbon, Portugal
                                    zoom: 6,
                                    zoomControl: true,
                                    attributionControl: true
                                });

                                // Add OpenStreetMap tiles (FREE - no API key needed!)
                                L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
                                    attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
                                    maxZoom: 19
                                }).addTo(map);

                                // Alternative tile layers you can use
                                const tileLayers = {
                                    street: L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
                                        attribution: '&copy; OpenStreetMap'
                                    }),
                                    satellite: L.tileLayer('https://server.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer/tile/{z}/{y}/{x}', {
                                        attribution: '&copy; Esri'
                                    }),
                                    terrain: L.tileLayer('https://{s}.tile.opentopomap.org/{z}/{x}/{y}.png', {
                                        attribution: '&copy; OpenTopoMap'
                                    }),
                                    dark: L.tileLayer('https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png', {
                                        attribution: '&copy; CartoDB'
                                    })
                                };

                                // Store globally
                                window.handymanMap = map;
                                window.tileLayers = tileLayers;

                                // Custom marker icon
                                const customIcon = L.divIcon({
                                    html: '<div style="background:#e74c3c;width:30px;height:30px;border-radius:50%;border:3px solid white;box-shadow:0 2px 5px rgba(0,0,0,0.3);"></div>',
                                    className: 'custom-marker',
                                    iconSize: [30, 30],
                                    iconAnchor: [15, 15]
                                });

                                // Service markers array
                                window.serviceMarkers = [];

                                // Function to add markers
                                window.addServiceMarker = function(lat, lng, title, description) {
                                    const marker = L.marker([lat, lng], {
                                        icon: customIcon,
                                        draggable: true
                                    }).addTo(map);

                                    marker.bindPopup(`
                                        <div style="padding:10px;">
                                            <h3 style="margin:0 0 8px 0;color:#2c3e50;">${title}</h3>
                                            <p style="margin:0;color:#666;">${description}</p>
                                            <button onclick="removeMarker(${window.serviceMarkers.length})"
                                                    style="margin-top:10px;padding:5px 10px;background:#e74c3c;color:white;border:none;border-radius:4px;cursor:pointer;">
                                                Remove
                                            </button>
                                        </div>
                                    `);

                                    marker.on('dragend', function(e) {
                                        const latlng = e.target.getLatLng();
                                        console.log('Marker moved to:', latlng);
                                    });

                                    window.serviceMarkers.push(marker);
                                    return marker;
                                };

                                window.removeMarker = function(index) {
                                    if (window.serviceMarkers[index]) {
                                        map.removeLayer(window.serviceMarkers[index]);
                                        window.serviceMarkers.splice(index, 1);
                                    }
                                };

                                window.clearAllMarkers = function() {
                                    window.serviceMarkers.forEach(marker => map.removeLayer(marker));
                                    window.serviceMarkers = [];
                                };

                                window.changeMapStyle = function(style) {
                                    // Remove current tiles
                                    map.eachLayer(function(layer) {
                                        if (layer instanceof L.TileLayer) {
                                            map.removeLayer(layer);
                                        }
                                    });

                                    // Add new style
                                    if (tileLayers[style]) {
                                        tileLayers[style].addTo(map);
                                    }
                                };

                                window.flyToLocation = function(lat, lng, zoom = 14) {
                                    map.flyTo([lat, lng], zoom, {
                                        duration: 2
                                    });
                                };

                                // Add default markers
                                window.addServiceMarker(38.7223, -9.1393, 'Lisbon Central', 'Main service hub in Lisbon');
                                window.addServiceMarker(38.7074, -9.1480, 'Belém Service', 'Historical district coverage');
                                window.addServiceMarker(41.1579, -8.6291, 'Porto Service', 'Porto area coverage');

                                // Add scale
                                L.control.scale().addTo(map);


                                console.log('Map initialized with OpenStreetMap - Portugal view with UI controls');

                                // Zoom to Portugal view with smooth animation
                                // Create UI control panels using DOM methods
const controlsContainer = document.getElementById('map-container').parentElement;

// Style Switcher Panel (top-right)
const stylePanel = document.createElement('div');
stylePanel.style.cssText = 'position:absolute;top:20px;right:20px;background:rgba(255,255,255,0.95);padding:15px;border-radius:10px;box-shadow:0 2px 10px rgba(0,0,0,0.2);z-index:1000;';
const styleTitle = document.createElement('h3');
styleTitle.textContent = 'Map Style';
styleTitle.style.cssText = 'margin:0 0 10px 0;color:#2c3e50;font-size:1rem;';
stylePanel.appendChild(styleTitle);

const styles = [
    {name: 'Street Map', style: 'street', color: '#3498db'},
    {name: 'Satellite', style: 'satellite', color: '#27ae60'},
    {name: 'Terrain', style: 'terrain', color: '#e67e22'},
    {name: 'Dark Mode', style: 'dark', color: '#34495e'}
];

styles.forEach(function(s) {
    const btn = document.createElement('button');
    btn.textContent = s.name;
    btn.style.cssText = 'display:block;width:100%;margin:5px 0;padding:8px;background:'+s.color+';color:white;border:none;border-radius:5px;cursor:pointer;font-size:13px;';
    btn.addEventListener('click', function() { window.changeMapStyle(s.style); });
    stylePanel.appendChild(btn);
});

controlsContainer.appendChild(stylePanel);

// Quick Jump Panel (bottom-right)
const jumpPanel = document.createElement('div');
jumpPanel.style.cssText = 'position:absolute;bottom:80px;right:20px;background:rgba(255,255,255,0.95);padding:15px;border-radius:10px;box-shadow:0 2px 10px rgba(0,0,0,0.2);z-index:1000;';
const jumpTitle = document.createElement('h3');
jumpTitle.textContent = 'Quick Jump';
jumpTitle.style.cssText = 'margin:0 0 10px 0;color:#2c3e50;font-size:1rem;';
jumpPanel.appendChild(jumpTitle);

const locations = [
    {name: 'Lisbon', lat: 38.7223, lng: -9.1393, zoom: 12, color: '#1abc9c'},
    {name: 'Porto', lat: 41.1579, lng: -8.6291, zoom: 12, color: '#1abc9c'},
    {name: 'Faro Zone', lat: 37.0194, lng: -7.9304, zoom: 11, color: '#e74c3c'},
    {name: 'View All', lat: 39.5, lng: -8, zoom: 7, color: '#16a085'}
];

locations.forEach(function(loc) {
    const btn = document.createElement('button');
    btn.textContent = loc.name;
    btn.style.cssText = 'display:block;width:100%;margin:5px 0;padding:8px;background:'+loc.color+';color:white;border:none;border-radius:5px;cursor:pointer;font-size:13px;';
    btn.addEventListener('click', function() { window.flyToLocation(loc.lat, loc.lng, loc.zoom); });
    jumpPanel.appendChild(btn);
});

controlsContainer.appendChild(jumpPanel);

// Marker Controls (bottom-left)
const markerPanel = document.createElement('div');
markerPanel.style.cssText = 'position:absolute;bottom:80px;left:20px;background:rgba(255,255,255,0.95);padding:15px;border-radius:10px;box-shadow:0 2px 10px rgba(0,0,0,0.2);z-index:1000;';
const markerTitle = document.createElement('h3');
markerTitle.textContent = 'Markers';
markerTitle.style.cssText = 'margin:0 0 10px 0;color:#2c3e50;font-size:1rem;';
markerPanel.appendChild(markerTitle);

const clearBtn = document.createElement('button');
clearBtn.textContent = 'Clear All';
clearBtn.style.cssText = 'display:block;width:100%;margin:5px 0;padding:8px;background:#e74c3c;color:white;border:none;border-radius:5px;cursor:pointer;font-size:13px;';
clearBtn.addEventListener('click', function() { window.clearAllMarkers(); });
markerPanel.appendChild(clearBtn);

const addBtn = document.createElement('button');
addBtn.textContent = 'Add Marker';
addBtn.style.cssText = 'display:block;width:100%;margin:5px 0;padding:8px;background:#9b59b6;color:white;border:none;border-radius:5px;cursor:pointer;font-size:13px;';
addBtn.addEventListener('click', function() { window.addServiceMarker(37.0194, -7.9304, 'New Service', 'Click to edit'); });
markerPanel.appendChild(addBtn);

controlsContainer.appendChild(markerPanel);

console.log('UI control panels added');
                                setTimeout(() => {
                                    map.flyTo([39.5, -8], 7, { duration: 2 });
                                }, 1000);
                            })();
                        "#));

                        // Append the script to the document body to execute it
                        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                            if let Some(body) = doc.body() {
                                let _ = body.append_child(&script_elem);
                            }
                        }
                    }

                    // Log confirmation to browser console
                    log("Leaflet map initialized with OpenStreetMap");
                }
            });

            // Prevent timeout from being cancelled when it goes out of scope
            timeout.forget();
            || {}  // Cleanup function (none needed)
        });
    }

    // CSS styling for the map page and UI controls
    let css = css!(r#"
        .map-page {
            width: 100%;
            height: calc(100vh - 60px);
            position: relative;
            background: #f0f0f0;
        }

        .map-container {
            width: 100%;
            height: 100%;
            border-radius: 0;
        }

        .map-info {
            position: absolute;
            top: 20px;
            left: 20px;
            background: rgba(255, 255, 255, 0.98);
            padding: 25px;
            border-radius: 12px;
            box-shadow: 0 4px 20px rgba(0,0,0,0.3);
            z-index: 1000;
            max-width: 380px;
            backdrop-filter: blur(10px);
        }

        .map-info h2 {
            margin: 0 0 15px 0;
            color: #2c3e50;
            font-size: 1.6rem;
            font-weight: 700;
        }

        .map-info p {
            margin: 8px 0;
            color: #555;
            line-height: 1.6;
            font-size: 0.95rem;
        }

        .map-info strong {
            color: #2c3e50;
            display: block;
            margin-top: 12px;
        }

        .map-info ul {
            margin: 10px 0;
            padding-left: 20px;
        }

        .map-info li {
            margin: 8px 0;
            color: #666;
            line-height: 1.5;
            font-size: 0.9rem;
        }

        .map-info code {
            background: #f0f0f0;
            padding: 2px 6px;
            border-radius: 3px;
            font-family: 'Courier New', monospace;
            font-size: 0.85rem;
            color: #e74c3c;
        }

        .map-controls {
            position: absolute;
            bottom: 50px;
            left: 20px;
            background: rgba(46, 204, 113, 0.95);
            color: white;
            padding: 18px 24px;
            border-radius: 12px;
            box-shadow: 0 4px 20px rgba(0,0,0,0.3);
            z-index: 1000;
            backdrop-filter: blur(10px);
        }

        .map-controls p {
            margin: 0;
            font-size: 1rem;
            font-weight: 600;
            display: flex;
            align-items: center;
            gap: 8px;
        }

        .map-controls p::before {
            content: "🗺️";
            font-size: 1.2rem;
        }

        @media (max-width: 768px) {
            .map-info {
                max-width: calc(100% - 40px);
                padding: 20px;
            }

            .map-info h2 {
                font-size: 1.3rem;
            }

            .map-controls {
                bottom: 20px;
                left: 20px;
                right: 20px;
                text-align: center;
            }
        }
    "#);

    // Render the map page HTML structure
    html! {
        <div class={css}>
            <div class="map-page">
                <div class="map-info">
                    <h2>{ "Portugal Interactive Map" }</h2>
                    <p>{ "OpenStreetMap - Free and fully programmable!" }</p>

                    <strong>{ "Map Features:" }</strong>
                    <ul>
                        <li>{ "🗺️ OpenStreetMap (no API key needed)" }</li>
                        <li>{ "🖱️ Click and drag to pan" }</li>
                        <li>{ "🔍 Scroll to zoom in/out" }</li>
                        <li>{ "📍 Draggable service markers" }</li>
                        <li>{ "🎨 Multiple map styles" }</li>
                    </ul>

                    <strong>{ "Programming Functions:" }</strong>
                    <ul>
                        <li><code>{ "changeMapStyle('satellite')" }</code></li>
                        <li><code>{ "addServiceMarker(lat, lng)" }</code></li>
                        <li><code>{ "flyToLocation(lat, lng)" }</code></li>
                        <li><code>{ "clearAllMarkers()" }</code></li>
                    </ul>
                    <p style="margin-top: 12px; font-size: 0.85rem; color: #7f8c8d;">
                        { "Open console (F12) to program the map!" }
                    </p>
                </div>

                <div
                    ref={map_container_ref}
                    id="map-container"
                    class="map-container"
                />

                <div class="map-controls">
                    <p>{ "Portugal - OpenStreetMap" }</p>
                </div>
            </div>
        </div>
    }
}

//! # Map Page (Cesium 3D)
//!
//! ## Purpose
//! Interactive 3D map using Cesium.js to display service locations.
//! Shows available handymen locations, service areas (future).
//!
//! ## How It Works
//! 1. Cesium.js loaded from CDN in index.html
//! 2. Component creates viewer on div element
//! 3. Loads Cesium Ion assets (3D globe, imagery)
//! 4. Sets initial camera position
//! 5. (Future) Displays handyman markers, service areas
//!
//! ## Features
//! - 3D globe with satellite imagery
//! - Interactive camera controls (pan, zoom, rotate)
//! - Future: Handyman markers, click for details
//!
//! ## Relation to Entire Program
//! - **Route**: `/map`
//! - **External Deps**: Cesium.js from CDN, Cesium Ion API
//! - **Public**: No authentication required

use yew::prelude::*;           // Yew framework
use stylist::css;               // CSS-in-Rust styling
use wasm_bindgen::prelude::*;   // WASM-JS bindings
use web_sys::HtmlElement;       // DOM element type
use gloo::timers::callback::Timeout;  // Async timeout for initialization

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[function_component]
pub fn MapPage() -> Html {
    let map_container_ref = use_node_ref();

    {
        let map_container_ref = map_container_ref.clone();

        use_effect_with((), move |_| {
            let timeout = Timeout::new(1000, move || {
                if let Some(_container) = map_container_ref.cast::<HtmlElement>() {
                    let script = web_sys::window()
                        .and_then(|w| w.document())
                        .and_then(|doc| doc.create_element("script").ok())
                        .and_then(|s| s.dyn_into::<web_sys::HtmlScriptElement>().ok());

                    if let Some(script_elem) = script {
                        script_elem.set_text_content(Some(r#"
(function() {
    function initCesiumMap() {
        console.log('Checking for Cesium and map container...');

        if (typeof Cesium === 'undefined') {
            console.log('Waiting for Cesium library to load...');
            setTimeout(initCesiumMap, 100);
            return;
        }

        const container = document.getElementById('map-container');
        if (!container) {
            console.log('Waiting for map container div...');
            setTimeout(initCesiumMap, 100);
            return;
        }

        console.log('Cesium loaded! Initializing 3D globe on container:', container);

        try {
            // Set Ion default access token to empty to prevent 401 errors
            Cesium.Ion.defaultAccessToken = '';

            const viewer = new Cesium.Viewer('map-container', {
                imageryProvider: false,
                baseLayerPicker: false,
                geocoder: false,
                homeButton: true,
                sceneModePicker: true,
                navigationHelpButton: false,
                animation: false,
                timeline: false,
                fullscreenButton: true,
                requestRenderMode: false
            });

            // Add OpenStreetMap as default - FREE and high quality
            const osmLayer = viewer.imageryLayers.addImageryProvider(
                new Cesium.OpenStreetMapImageryProvider({
                    url: 'https://a.tile.openstreetmap.org/'
                })
            );

            viewer.scene.globe.enableLighting = true;
            viewer.scene.globe.depthTestAgainstTerrain = false;

        const location = Cesium.Cartesian3.fromDegrees(-7.8869, 37.1526, 5000);

        viewer.camera.flyTo({
            destination: location,
            orientation: {
                heading: Cesium.Math.toRadians(0),
                pitch: Cesium.Math.toRadians(-45),
                roll: 0.0
            },
            duration: 3
        });

        viewer.entities.add({
            id: 'coverage-zone',
            name: '10km Coverage Zone',
            position: Cesium.Cartesian3.fromDegrees(-7.8869, 37.1526),
            ellipse: {
                semiMinorAxis: 10000.0,
                semiMajorAxis: 10000.0,
                height: 0,
                material: Cesium.Color.BLUE.withAlpha(0.3),
                outline: true,
                outlineColor: Cesium.Color.BLUE,
                outlineWidth: 3
            }
        });

        viewer.entities.add({
            id: 'hq-marker',
            name: 'HQ - São Brás de Alportel',
            position: Cesium.Cartesian3.fromDegrees(-7.8869, 37.1526, 100),
            point: {
                pixelSize: 15,
                color: Cesium.Color.RED,
                outlineColor: Cesium.Color.WHITE,
                outlineWidth: 3
            },
            label: {
                text: 'HQ',
                font: '14pt sans-serif',
                style: Cesium.LabelStyle.FILL_AND_OUTLINE,
                outlineWidth: 2,
                verticalOrigin: Cesium.VerticalOrigin.BOTTOM,
                pixelOffset: new Cesium.Cartesian2(0, -20)
            }
        });

            // Create interactive buttons for switching imagery
            const controlsDiv = document.createElement('div');
            controlsDiv.id = 'map-controls';
            controlsDiv.innerHTML = `
                <div class="control-panel">
                    <h3>Map Controls</h3>
                    <div class="button-group">
                        <button class="map-btn active" data-layer="osm">Street Map</button>
                        <button class="map-btn" data-layer="topo">Topographic</button>
                        <button class="map-btn" data-layer="dark">Dark Mode</button>
                        <button class="map-btn" data-layer="light">Light Mode</button>
                    </div>
                    <div class="button-group">
                        <button class="map-btn" data-layer="satellite">Satellite</button>
                        <button class="map-btn" data-layer="hybrid">Hybrid</button>
                    </div>
                    <div class="coverage-toggle">
                        <label>
                            <input type="checkbox" id="toggle-coverage" checked>
                            Show 10km Coverage
                        </label>
                    </div>
                </div>
            `;
            document.getElementById('map-container').appendChild(controlsDiv);

            // Button click handlers
            const buttons = controlsDiv.querySelectorAll('.map-btn');
            buttons.forEach(btn => {
                btn.addEventListener('click', function() {
                    buttons.forEach(b => b.classList.remove('active'));
                    this.classList.add('active');

                    const layer = this.getAttribute('data-layer');

                    try {
                        viewer.imageryLayers.removeAll();

                    switch(layer) {
                        case 'osm':
                            viewer.imageryLayers.addImageryProvider(
                                new Cesium.OpenStreetMapImageryProvider({
                                    url: 'https://tile.openstreetmap.org/'
                                })
                            );
                            break;
                        case 'topo':
                            viewer.imageryLayers.addImageryProvider(
                                new Cesium.UrlTemplateImageryProvider({
                                    url: 'https://{s}.tile.opentopomap.org/{z}/{x}/{y}.png',
                                    subdomains: ['a', 'b', 'c'],
                                    credit: 'OpenTopoMap'
                                })
                            );
                            break;
                        case 'dark':
                            viewer.imageryLayers.addImageryProvider(
                                new Cesium.UrlTemplateImageryProvider({
                                    url: 'https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}.png',
                                    subdomains: ['a', 'b', 'c', 'd'],
                                    credit: 'CartoDB Dark'
                                })
                            );
                            break;
                        case 'light':
                            viewer.imageryLayers.addImageryProvider(
                                new Cesium.UrlTemplateImageryProvider({
                                    url: 'https://{s}.basemaps.cartocdn.com/light_all/{z}/{x}/{y}.png',
                                    subdomains: ['a', 'b', 'c', 'd'],
                                    credit: 'CartoDB Light'
                                })
                            );
                            break;
                        case 'satellite':
                            viewer.imageryLayers.addImageryProvider(
                                new Cesium.ArcGisMapServerImageryProvider({
                                    url: 'https://services.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer'
                                })
                            );
                            break;
                        case 'hybrid':
                            viewer.imageryLayers.addImageryProvider(
                                new Cesium.ArcGisMapServerImageryProvider({
                                    url: 'https://services.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer'
                                })
                            );
                            viewer.imageryLayers.addImageryProvider(
                                new Cesium.UrlTemplateImageryProvider({
                                    url: 'https://{s}.basemaps.cartocdn.com/rastertiles/voyager_only_labels/{z}/{x}/{y}.png',
                                    subdomains: ['a', 'b', 'c', 'd'],
                                    credit: 'Labels'
                                })
                            );
                            break;
                        default:
                            console.error('Unknown layer:', layer);
                    }
                    } catch (error) {
                        console.error('Error switching map layer:', error);
                    }
                });
            });

            // Coverage toggle handler
            document.getElementById('toggle-coverage').addEventListener('change', function(e) {
                const coverage = viewer.entities.getById('coverage-zone');
                const hq = viewer.entities.getById('hq-marker');
                if (coverage) coverage.show = e.target.checked;
                if (hq) hq.show = e.target.checked;
            });

            window.cesiumViewer = viewer;
            console.log('Cesium 3D globe ready! Earth should be visible now with OpenStreetMap.');
        } catch (error) {
            console.error('Error initializing Cesium viewer:', error);
        }
    }

    initCesiumMap();
})();
                        "#));

                        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                            if let Some(body) = doc.body() {
                                let _ = body.append_child(&script_elem);
                            }
                        }
                    }

                    log("Cesium 3D Globe initialized!");
                }
            });

            timeout.forget();
            || {}
        });
    }

    let css = css!(r#"
        .map-page {
            width: 100%;
            height: calc(100vh - 60px);
            position: relative;
            background: #000;
        }

        .map-container {
            width: 100%;
            height: 100%;
        }

        #map-controls {
            position: absolute;
            top: 20px;
            right: 20px;
            z-index: 1000;
        }

        .control-panel {
            background: rgba(255, 255, 255, 0.98);
            padding: 25px;
            border-radius: 12px;
            box-shadow: 0 8px 24px var(--shadow-dark);
            min-width: 300px;
            border: 3px solid var(--primary-red);
        }

        .control-panel h3 {
            margin: 0 0 20px 0;
            color: var(--primary-red);
            font-size: 20px;
            font-weight: 800;
            border-bottom: 3px solid var(--primary-red);
            padding-bottom: 12px;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }

        .button-group {
            display: flex;
            flex-direction: column;
            gap: 10px;
            margin-bottom: 18px;
        }

        .map-btn {
            padding: 12px 18px;
            background: var(--pure-white);
            border: 2px solid var(--primary-red);
            color: var(--text-dark);
            border-radius: 8px;
            cursor: pointer;
            font-size: 15px;
            font-weight: 600;
            transition: all 0.3s ease;
            text-align: left;
            box-shadow: 0 2px 6px var(--shadow-light);
        }

        .map-btn:hover {
            background: var(--light-gray);
            border-color: var(--dark-red);
            transform: translateX(5px);
            box-shadow: 0 4px 12px var(--shadow-medium);
        }

        .map-btn.active {
            background: var(--primary-red);
            color: white;
            font-weight: 700;
            transform: translateX(5px);
            box-shadow: 0 6px 16px var(--shadow-medium);
        }

        .coverage-toggle {
            margin-top: 15px;
            padding-top: 15px;
            border-top: 1px solid #ddd;
        }

        .coverage-toggle label {
            display: flex;
            align-items: center;
            gap: 8px;
            cursor: pointer;
            color: #555;
            font-size: 14px;
        }

        .coverage-toggle input[type="checkbox"] {
            width: 18px;
            height: 18px;
            cursor: pointer;
        }
    "#);

    html! {
        <div class={css}>
            <div class="map-page">
                <div
                    ref={map_container_ref}
                    id="map-container"
                    class="map-container"
                />
            </div>
        </div>
    }
}

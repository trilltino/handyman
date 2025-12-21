use leptos::prelude::*;
use leptos_meta::{Link, Script};
use leptos_router::hooks::use_params_map;

#[component]
pub fn HandymanServiceDetail() -> impl IntoView {
    let params = use_params_map();
    let service_slug = move || params.get().get("slug").unwrap_or_default();

    // Map initialization script
    let map_init_script = "
        function initMap() {
            if (!window.maplibregl) { setTimeout(initMap, 100); return; }
            if (!document.getElementById('map')) { setTimeout(initMap, 100); return; }
            
            const map = new maplibregl.Map({
                container: 'map',
                style: 'https://demotiles.maplibre.org/style.json', // Free open source style
                center: [-1.5197, 52.4068], // Coventry
                zoom: 13,
                pitch: 45,
                bearing: -17.6,
                antialias: true
            });

            map.on('load', () => {
                // Add 3D buildings
                const layers = map.getStyle().layers;
                let labelLayerId;
                for (let i = 0; i < layers.length; i++) {
                    if (layers[i].type === 'symbol' && layers[i].layout['text-field']) {
                        labelLayerId = layers[i].id;
                        break;
                    }
                }

                map.addSource('openmaptiles', {
                    url: 'https://demotiles.maplibre.org/tiles/v3/tiles.json',
                    type: 'vector',
                });

                map.addLayer(
                    {
                        'id': '3d-buildings',
                        'source': 'openmaptiles',
                        'source-layer': 'building',
                        'filter': ['==', 'extrude', 'true'],
                        'type': 'fill-extrusion',
                        'minzoom': 13,
                        'paint': {
                            'fill-extrusion-color': '#aaa',
                            'fill-extrusion-height': [
                                'interpolate',
                                ['linear'],
                                ['zoom'],
                                13,
                                0,
                                13.05,
                                ['get', 'height']
                            ],
                            'fill-extrusion-base': [
                                'interpolate',
                                ['linear'],
                                ['zoom'],
                                13,
                                0,
                                13.05,
                                ['get', 'min_height']
                            ],
                            'fill-extrusion-opacity': 0.6
                        }
                    },
                    labelLayerId
                );
            });
        }
        // Start checking for lib/dom
        initMap();
    ";

    view! {
        // Load MapLibre Assets
        <Link href="https://unpkg.com/maplibre-gl@4.7.1/dist/maplibre-gl.css" rel="stylesheet"/>
        <Script src="https://unpkg.com/maplibre-gl@4.7.1/dist/maplibre-gl.js"/>
        <Script>{map_init_script}</Script>

        <div class="bg-gray-50 min-h-screen">
            <section class="bg-blue-900 text-white py-12 px-6">
                <div class="max-w-6xl mx-auto">
                    <a href="/handyman-coventry/services" class="text-yellow-400 hover:text-yellow-300 text-sm font-bold uppercase tracking-wide mb-4 inline-block">"← Back to Services"</a>
                    <h1 class="text-4xl font-bold uppercase mb-2">"Service: " {service_slug}</h1>
                    <p class="text-blue-200 text-lg">"Professional solutions for your home."</p>
                </div>
            </section>

            <section class="py-12 px-6 max-w-6xl mx-auto">
                <div class="grid md:grid-cols-3 gap-8">
                    // Main Content
                    <div class="md:col-span-2 space-y-8">
                        <div class="bg-white p-8 rounded-xl shadow-sm border border-gray-100">
                             <h2 class="text-2xl font-bold text-gray-900 mb-4">"Service Overview"</h2>
                             <p class="text-gray-600 leading-relaxed mb-4">
                                "We provide top-tier " {service_slug} " services across Coventry. Our team is fully equipped to handle all aspects of the job, ensuring a high-quality finish every time."
                             </p>
                             <ul class="list-disc list-inside text-gray-600 space-y-2">
                                <li>"Qualified professionals"</li>
                                <li>"Fully insured"</li>
                                <li>"Competitive rates"</li>
                                <li>"Satisfaction guaranteed"</li>
                             </ul>
                        </div>

                         // 3D Map Container
                         <div class="bg-white p-2 rounded-xl shadow-sm border border-gray-100 overflow-hidden">
                            <div class="bg-gray-100 w-full h-[400px] flex items-center justify-center relative">
                                <div id="map" class="w-full h-full"></div>
                            </div>
                            <div class="p-4 bg-white text-xs text-gray-500 text-center border-t border-gray-100 italic">
                                "Interactive 3D Map of Service Area (Coventry)"
                            </div>
                        </div>
                    </div>

                    // Sidebar
                    <aside class="space-y-6">
                        <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100">
                            <h3 class="font-bold text-gray-900 mb-4">"Book This Service"</h3>
                            <a href="tel:07833263486" class="block w-full text-center py-3 bg-yellow-500 text-blue-900 font-bold rounded hover:bg-yellow-400 transition">"Call Now"</a>
                        </div>
                    </aside>
                </div>
            </section>
        </div>
    }
}

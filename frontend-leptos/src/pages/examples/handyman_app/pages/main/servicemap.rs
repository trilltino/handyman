//! 3D Service Area Map page - MAXIMUM QUALITY.
//!
//! Interactive 3D globe with Bing Maps satellite imagery, 3D terrain, OSM Buildings, and enhanced rendering.

use leptos::prelude::*;

#[component]
pub fn HandymanServiceMap() -> impl IntoView {
    view! {
        <>
            // Cesium CSS and JS
            <link rel="stylesheet" href="https://cesium.com/downloads/cesiumjs/releases/1.113/Build/Cesium/Widgets/widgets.css" />
            <script src="https://cesium.com/downloads/cesiumjs/releases/1.113/Build/Cesium/Cesium.js"></script>

            <div class="min-h-screen bg-gradient-to-b from-blue-900 via-blue-950 to-slate-900">
                // Header
                <section class="pt-8 pb-4 px-4 text-center">
                    <h1 class="text-4xl md:text-5xl font-black text-white mb-4">
                        "Our " <span class="text-yellow-400">"Service Area"</span>
                    </h1>
                    <p class="text-blue-200 text-lg max-w-2xl mx-auto">
                        "Explore our coverage area on the interactive 3D map with satellite imagery and real buildings."
                    </p>
                </section>

                // Controls Panel
                <section class="px-4 pb-4">
                    <div class="max-w-6xl mx-auto">
                        <div class="bg-blue-900/60 backdrop-blur-md rounded-xl p-4 border border-blue-500/30 shadow-lg">
                            <div class="flex flex-wrap gap-4 items-center justify-between">
                                <div class="flex flex-wrap gap-2">
                                    <span class="text-blue-200 text-sm font-bold mr-2 self-center">"Jump to:"</span>
                                    <button id="btn-coventry" class="px-4 py-2 bg-yellow-500 text-blue-900 rounded-lg font-bold text-sm hover:bg-yellow-400 transition shadow-md">"Coventry (HQ)"</button>
                                    <button id="btn-birmingham" class="px-4 py-2 bg-blue-600 text-white rounded-lg font-bold text-sm hover:bg-blue-500 transition shadow-md">"Birmingham"</button>
                                    <button id="btn-warwick" class="px-4 py-2 bg-blue-600 text-white rounded-lg font-bold text-sm hover:bg-blue-500 transition shadow-md">"Warwick"</button>
                                    <button id="btn-rugby" class="px-4 py-2 bg-blue-600 text-white rounded-lg font-bold text-sm hover:bg-blue-500 transition shadow-md">"Rugby"</button>
                                    <button id="btn-all" class="px-4 py-2 bg-blue-600 text-white rounded-lg font-bold text-sm hover:bg-blue-500 transition shadow-md">"All Areas"</button>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                // Cesium Container
                <section class="relative px-4 pb-8">
                    <div class="max-w-6xl mx-auto">
                        <div id="cesiumContainer" class="w-full h-[500px] md:h-[600px] rounded-2xl overflow-hidden border-2 border-blue-500/30 shadow-2xl shadow-blue-500/20"></div>
                    </div>
                </section>

                // Legend
                <section class="py-8 px-4">
                    <div class="max-w-4xl mx-auto">
                        <div class="grid md:grid-cols-3 gap-6">
                            <div class="bg-blue-900/50 backdrop-blur-sm rounded-xl p-6 border border-blue-500/20">
                                <div class="flex items-center gap-3 mb-4">
                                    <div class="w-5 h-5 rounded-full bg-yellow-400 shadow-lg shadow-yellow-400/50"></div>
                                    <h3 class="text-lg font-bold text-white">"Main Office"</h3>
                                </div>
                                <p class="text-blue-200 text-sm">"Coventry - Our headquarters"</p>
                            </div>
                            <div class="bg-blue-900/50 backdrop-blur-sm rounded-xl p-6 border border-blue-500/20">
                                <div class="flex items-center gap-3 mb-4">
                                    <div class="w-5 h-5 rounded-full bg-blue-400 shadow-lg shadow-blue-400/50"></div>
                                    <h3 class="text-lg font-bold text-white">"Service Areas"</h3>
                                </div>
                                <p class="text-blue-200 text-sm">"8 towns across West Midlands"</p>
                            </div>
                            <div class="bg-blue-900/50 backdrop-blur-sm rounded-xl p-6 border border-blue-500/20">
                                <div class="flex items-center gap-3 mb-4">
                                    <svg class="w-5 h-5 text-blue-400" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
                                    </svg>
                                    <h3 class="text-lg font-bold text-white">"3D Buildings"</h3>
                                </div>
                                <p class="text-blue-200 text-sm">"Real building data"</p>
                            </div>
                        </div>
                    </div>
                </section>

                // CTA
                <section class="py-12 px-4 text-center">
                    <div class="max-w-2xl mx-auto">
                        <h2 class="text-2xl font-bold text-white mb-4">"In Our Service Area?"</h2>
                        <p class="text-blue-200 mb-8">"We're ready to help with all your handyman needs across the West Midlands!"</p>
                        <a href="/handyman-coventry/booking" class="inline-block px-8 py-4 bg-gradient-to-r from-yellow-400 to-yellow-500 text-blue-900 rounded-lg font-bold text-lg hover:shadow-lg hover:shadow-yellow-500/30 hover:-translate-y-1 transition">"Book Now"</a>
                    </div>
                </section>
            </div>

            // CesiumJS MAXIMUM QUALITY initialization
            <script>
                r#"
                (function() {
                    var attempts = 0;
                    
                    function initCesium() {
                        attempts++;
                        if (typeof Cesium === 'undefined') {
                            if (attempts < 50) setTimeout(initCesium, 200);
                            return;
                        }
                        
                        var container = document.getElementById('cesiumContainer');
                        if (!container || container.children.length > 0) return;
                        
                        // Cesium ion access token
                        Cesium.Ion.defaultAccessToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJqdGkiOiJjZDFjMTJjYy0zMDU5LTQyYjktYjA0OS01MzAwNTMyZWJiNGEiLCJpZCI6MzcyMjQzLCJpYXQiOjE3NjY1MjM3MjR9.34z60p_cRn7YNRMAENXDOvG4vMa1zP1MLo1ApjaQbKA';
                        
                        // Load Bing Maps Aerial with Labels (highest quality satellite + labels)
                        Cesium.IonImageryProvider.fromAssetId(3).then(function(imageryProvider) {
                            var viewer = new Cesium.Viewer('cesiumContainer', {
                                baseLayer: new Cesium.ImageryLayer(imageryProvider),
                                animation: false,
                                timeline: false,
                                homeButton: false,
                                sceneModePicker: false,
                                baseLayerPicker: false,
                                navigationHelpButton: false,
                                fullscreenButton: true,
                                geocoder: false,
                                infoBox: true,
                                selectionIndicator: true,
                                shadows: true,
                                requestRenderMode: false
                            });
                            
                            // MAXIMUM QUALITY SETTINGS
                            viewer.scene.globe.maximumScreenSpaceError = 1; // Highest detail (default is 2)
                            viewer.scene.globe.tileCacheSize = 1000; // More cached tiles
                            viewer.scene.globe.preloadSiblings = true; // Preload nearby tiles
                            viewer.scene.globe.preloadAncestors = true; // Preload parent tiles
                            viewer.scene.globe.enableLighting = true; // Realistic sun lighting
                            viewer.scene.highDynamicRange = true; // HDR rendering
                            viewer.scene.msaaSamples = 4; // Anti-aliasing (4x MSAA)
                            viewer.scene.fxaa = true; // Fast approximate anti-aliasing
                            viewer.scene.postProcessStages.fxaa.enabled = true;
                            viewer.scene.globe.depthTestAgainstTerrain = true;
                            viewer.resolutionScale = window.devicePixelRatio || 1; // Match display resolution
                            
                            // High-quality atmosphere
                            viewer.scene.skyAtmosphere.hueShift = -0.05;
                            viewer.scene.skyAtmosphere.saturationShift = 0.1;
                            viewer.scene.skyAtmosphere.brightnessShift = 0.1;
                            
                            // Add Cesium World Terrain (high-resolution elevation)
                            Cesium.CesiumTerrainProvider.fromIonAssetId(1).then(function(terrainProvider) {
                                viewer.terrainProvider = terrainProvider;
                                viewer.scene.globe.terrainExaggeration = 1.0;
                            });
                            
                            // Add OSM Buildings 3D (real 3D buildings worldwide)
                            Cesium.Cesium3DTileset.fromIonAssetId(96188).then(function(tileset) {
                                viewer.scene.primitives.add(tileset);
                                // Style buildings
                                tileset.style = new Cesium.Cesium3DTileStyle({
                                    color: "color('white', 0.9)",
                                    show: true
                                });
                            }).catch(function(err) {
                                console.log('3D buildings not available:', err);
                            });
                            
                            // Service locations
                            var locations = [
                                { name: 'Coventry (HQ)', lat: 52.4068, lng: -1.5197, isHQ: true, desc: 'Our main headquarters' },
                                { name: 'Birmingham', lat: 52.4862, lng: -1.8904, isHQ: false, desc: 'Full service coverage' },
                                { name: 'Warwick', lat: 52.2852, lng: -1.5849, isHQ: false, desc: 'Historic town coverage' },
                                { name: 'Leamington Spa', lat: 52.2852, lng: -1.5317, isHQ: false, desc: 'Regular service area' },
                                { name: 'Nuneaton', lat: 52.5233, lng: -1.4684, isHQ: false, desc: 'North coverage zone' },
                                { name: 'Rugby', lat: 52.3708, lng: -1.2619, isHQ: false, desc: 'East coverage zone' },
                                { name: 'Solihull', lat: 52.4130, lng: -1.7780, isHQ: false, desc: 'Premium service area' },
                                { name: 'Kenilworth', lat: 52.3414, lng: -1.5692, isHQ: false, desc: 'Close to HQ' },
                                { name: 'Stratford-upon-Avon', lat: 52.1917, lng: -1.7083, isHQ: false, desc: 'South coverage zone' }
                            ];
                            
                            // Add high-quality markers with billboards
                            for (var i = 0; i < locations.length; i++) {
                                var loc = locations[i];
                                viewer.entities.add({
                                    name: loc.name,
                                    description: '<p style="font-size:14px;">' + loc.desc + '</p>',
                                    position: Cesium.Cartesian3.fromDegrees(loc.lng, loc.lat, 50),
                                    point: {
                                        pixelSize: loc.isHQ ? 28 : 20,
                                        color: loc.isHQ ? Cesium.Color.YELLOW : Cesium.Color.DODGERBLUE,
                                        outlineColor: Cesium.Color.WHITE,
                                        outlineWidth: 4,
                                        heightReference: Cesium.HeightReference.CLAMP_TO_GROUND,
                                        disableDepthTestDistance: Number.POSITIVE_INFINITY
                                    },
                                    label: {
                                        text: loc.name,
                                        font: 'bold 16px sans-serif',
                                        fillColor: Cesium.Color.WHITE,
                                        outlineColor: Cesium.Color.BLACK,
                                        outlineWidth: 4,
                                        style: Cesium.LabelStyle.FILL_AND_OUTLINE,
                                        verticalOrigin: Cesium.VerticalOrigin.BOTTOM,
                                        pixelOffset: new Cesium.Cartesian2(0, -35),
                                        disableDepthTestDistance: Number.POSITIVE_INFINITY,
                                        scaleByDistance: new Cesium.NearFarScalar(1000, 1.2, 50000, 0.6)
                                    }
                                });
                                
                                // Glowing lines from HQ
                                if (!loc.isHQ) {
                                    viewer.entities.add({
                                        polyline: {
                                            positions: Cesium.Cartesian3.fromDegreesArrayHeights([
                                                -1.5197, 52.4068, 500,
                                                loc.lng, loc.lat, 500
                                            ]),
                                            width: 5,
                                            material: new Cesium.PolylineGlowMaterialProperty({
                                                glowPower: 0.4,
                                                taperPower: 0.5,
                                                color: Cesium.Color.YELLOW.withAlpha(0.9)
                                            })
                                        }
                                    });
                                }
                            }
                            
                            // Coverage circle
                            viewer.entities.add({
                                position: Cesium.Cartesian3.fromDegrees(-1.5197, 52.4068),
                                ellipse: {
                                    semiMajorAxis: 25000,
                                    semiMinorAxis: 25000,
                                    material: Cesium.Color.YELLOW.withAlpha(0.08),
                                    outline: true,
                                    outlineColor: Cesium.Color.YELLOW.withAlpha(0.5),
                                    outlineWidth: 3,
                                    heightReference: Cesium.HeightReference.CLAMP_TO_GROUND
                                }
                            });
                            
                            // Smooth fly-to with optimal pitch
                            function flyTo(lng, lat, height, pitch) {
                                viewer.camera.flyTo({
                                    destination: Cesium.Cartesian3.fromDegrees(lng, lat, height),
                                    orientation: {
                                        heading: Cesium.Math.toRadians(0),
                                        pitch: Cesium.Math.toRadians(pitch || -45),
                                        roll: 0
                                    },
                                    duration: 2.5,
                                    easingFunction: Cesium.EasingFunction.QUADRATIC_IN_OUT
                                });
                            }
                            
                            // Initial view - overview
                            flyTo(-1.5197, 52.4068, 50000, -35);
                            
                            // Button handlers - zoom to street level for max detail
                            document.getElementById('btn-coventry').onclick = function() { flyTo(-1.5197, 52.4068, 2000, -50); };
                            document.getElementById('btn-birmingham').onclick = function() { flyTo(-1.8904, 52.4862, 3000, -50); };
                            document.getElementById('btn-warwick').onclick = function() { flyTo(-1.5849, 52.2852, 2000, -50); };
                            document.getElementById('btn-rugby').onclick = function() { flyTo(-1.2619, 52.3708, 2000, -50); };
                            document.getElementById('btn-all').onclick = function() { flyTo(-1.55, 52.38, 80000, -30); };
                            
                            console.log('Cesium MAX QUALITY initialized: Bing Aerial + Labels, 3D Terrain, OSM Buildings, HDR, 4x MSAA');
                            
                        }).catch(function(error) {
                            console.error('Failed to load Cesium:', error);
                        });
                    }
                    
                    if (document.readyState === 'complete') {
                        setTimeout(initCesium, 500);
                    } else {
                        window.addEventListener('load', function() { setTimeout(initCesium, 500); });
                    }
                })();
                "#
            </script>
        </>
    }
}

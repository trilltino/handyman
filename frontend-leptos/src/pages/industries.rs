use crate::components::seo::SeoHead;
use leptos::prelude::*;
use shared::PageMetadata;

#[derive(Clone, Default)]
struct IndustrySpec {
    title: String,
    booster: String,
    details: Vec<String>,
}

#[component]
pub fn Industries() -> impl IntoView {
    let active_spec = RwSignal::new(None::<IndustrySpec>);

    view! {
        <SeoHead metadata=PageMetadata {
            title: "Web Development for Tradesman Services | XF Tradesmen".to_string(),
            description: "High-performance web development and SEO for plumbers, electricians, and builders. Dominate local search for high-value jobs with our visibility engine.".to_string(),
            canonical_url: Some("https://xftradesman.com/industries".to_string()),
            og_image: None,
        }/>

        <div class="relative min-h-screen bg-void text-white overflow-x-hidden">
            // Modal Overlay
            <div
                class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-void/90 backdrop-blur-sm transition-all duration-300"
                class:opacity-0=move || active_spec.get().is_none()
                class:pointer-events-none=move || active_spec.get().is_none()
                on:click=move |_| active_spec.set(None)
            >
                <div
                    class="bg-void-surface border border-brand/50 p-8 rounded-2xl max-w-lg w-full shadow-2xl transform transition-all duration-500 delay-75"
                    class:scale-95=move || active_spec.get().is_none()
                    class:scale-100=move || active_spec.get().is_some()
                    class:opacity-0=move || active_spec.get().is_none()
                    on:click=move |e| e.stop_propagation()
                >
                    {move || active_spec.get().map(|spec| view! {
                        <div class="space-y-6">
                            <div class="flex justify-between items-start">
                                <div>
                                    <h3 class="text-3xl font-black text-white tracking-tighter mb-2">{spec.title.clone()}</h3>
                                    <div class="text-brand font-mono text-xs uppercase tracking-widest">{spec.booster.clone()}</div>
                                </div>
                                <button on:click=move |_| active_spec.set(None) class="text-gray-500 hover:text-white transition">
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/></svg>
                                </button>
                            </div>

                            <div class="space-y-4">
                                <div class="text-gray-400 text-sm font-light leading-relaxed italic">
                                    "Engineering custom SEO infrastructure for specific " {spec.title.clone()} " high-value leads..."
                                </div>
                                <ul class="space-y-3">
                                    {spec.details.into_iter().map(|item| view! {
                                        <li class="flex items-center gap-3 text-white/90 text-sm">
                                            <span class="w-1 h-1 bg-brand rounded-full"></span>
                                            {item}
                                        </li>
                                    }).collect_view()}
                                </ul>
                            </div>

                            <a href="/pricing" class="btn btn-primary w-full justify-center">"INITIALIZE THIS SPEC"</a>
                        </div>
                    })}
                </div>
            </div>

            // Hero Section
            <section class="relative border-b border-void-highlight py-32 px-4 overflow-hidden">
                <div class="absolute inset-0 bg-cyber-grid bg-[length:40px_40px] opacity-20"></div>
                <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[800px] h-[800px] bg-brand/5 blur-[120px] rounded-full pointer-events-none"></div>

                <div class="max-w-6xl mx-auto grid lg:grid-cols-2 gap-12 items-center relative z-10">
                    <div class="animate-fade-in">
                        <span class="text-brand font-mono text-sm tracking-widest uppercase mb-4 block">"System Architecture // Industries"</span>
                        <h1 class="text-5xl md:text-7xl font-heading font-black mb-6 tracking-tighter leading-[0.9]">
                            "WEB DEVELOPMENT" <br/>
                            <span class="text-brand">"FOR TRADESMAN SERVICES"</span>
                        </h1>
                        <p class="text-xl text-gray-400 max-w-xl font-light mb-8">
                            "Most agency sites are generic. We build <span class='text-white font-semibold'>Visibility Engines</span> tailored to the specific jobs that keep your business profitable."
                        </p>
                        <div class="flex gap-4">
                            <a href="/pricing" class="btn btn-primary">"BUILD MY ENGINE"</a>
                            <a href="#visibility-engine" class="btn btn-secondary">"HOW IT WORKS"</a>
                        </div>
                    </div>

                    <div class="relative group hidden lg:block">
                        <div class="absolute -inset-4 bg-brand/20 blur-3xl rounded-full opacity-0 group-hover:opacity-100 transition-opacity duration-1000"></div>
                        <img
                            src="/web_development_for_trades_hero_1766977280951.png"
                            alt="Elite Engineering for Manual Trades"
                            class="relative w-full h-auto rounded-2xl border border-void-highlight/50 shadow-2xl transform group-hover:scale-[1.02] transition-transform duration-700"
                        />
                    </div>
                </div>
            </section>

            // The Visibility Machine Section
            <section id="visibility-engine" class="py-24 px-4 bg-void-surface relative">
                <div class="max-w-6xl mx-auto">
                    <div class="text-center mb-16">
                        <h2 class="text-3xl md:text-5xl font-bold text-white mb-6 font-heading tracking-tight">"THE VISIBILITY MACHINE"</h2>
                        <p class="text-gray-400 max-w-2xl mx-auto">"How we translate your skills into high-value digital leads using our proprietary SEO mapping."</p>
                        <div class="h-1 w-24 bg-brand mx-auto mt-8 rounded-full"></div>
                    </div>

                    <div class="grid md:grid-cols-3 gap-8">
                        <div class="card-deep border-void-highlight/50">
                            <div class="text-brand font-mono text-4xl font-black mb-4">"01"</div>
                            <h3 class="text-xl font-bold text-white mb-3">"Job Extraction"</h3>
                            <p class="text-gray-400">"We don't just target 'Plumber'. we target 'Emergency Boiler Repair' and 'LPG Gas Certification'—the jobs that actually pay."</p>
                        </div>
                        <div class="card-deep border-void-highlight/50">
                            <div class="text-brand font-mono text-4xl font-black mb-4">"02"</div>
                            <h3 class="text-xl font-bold text-white mb-3">"Geo-Mapping"</h3>
                            <p class="text-gray-400">"Every service is anchored to your specific catchment area, ensuring you rank #1 for local 'near me' searches."</p>
                        </div>
                        <div class="card-deep border-void-highlight/50">
                            <div class="text-brand font-mono text-4xl font-black mb-4">"03"</div>
                            <h3 class="text-xl font-bold text-white mb-3">"Conversion Flow"</h3>
                            <p class="text-gray-400">"Traffic is routed through booking-optimized landing pages that turn browsers into confirmed enquiries."</p>
                        </div>
                    </div>
                </div>
            </section>

            // Industry Grid
            <section class="py-24 px-4 bg-void">
                <div class="max-w-7xl mx-auto">
                    <div class="mb-16">
                        <span class="text-brand font-mono text-sm tracking-widest uppercase mb-2 block">"Industry Templates"</span>
                        <h2 class="text-4xl font-bold text-white tracking-tighter">"SPECIALIZED VISIBILITY"</h2>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                        <IndustryCard
                            title="Electricians"
                            visibility="EV Charger Installs // EICR Checks"
                            description="Focus on high-value safety certifications and new tech installations."
                            image="/images/industries/industry_electrician.png"
                            on_view_spec=move || active_spec.set(Some(IndustrySpec {
                                title: "Electricians".to_string(),
                                booster: "EV Charger Installs // EICR Checks".to_string(),
                                details: vec![
                                    "Safe Electrician Schema Injection".to_string(),
                                    "EV Station Lead Magnets".to_string(),
                                    "Emergency Call-Out Landing Pages".to_string(),
                                    "24/7 Response Optimization".to_string(),
                                ]
                            }))
                        />
                        <IndustryCard
                            title="Plumbers"
                            visibility="Boiler Servicing // Leak Detection"
                            description="Highly conversion-focused for urgent repairs. Specific schema for Gas Safe."
                            image="/images/industries/industry_plumber.png"
                            on_view_spec=move || active_spec.set(Some(IndustrySpec {
                                title: "Plumbers".to_string(),
                                booster: "Boiler Servicing // Leak Detection".to_string(),
                                details: vec![
                                    "Gas Safe Badge Integration".to_string(),
                                    "Emergency Pipelink Infrastructure".to_string(),
                                    "Boiler Recurring Service Funnels".to_string(),
                                    "Local Maps Protection Layer".to_string(),
                                ]
                            }))
                        />
                        <IndustryCard
                            title="Carpenters"
                            visibility="Bespoke Joinery // Kitchen Fitting"
                            description="Visual-heavy portfolios that showcase craftsmanship and renovation."
                            image="/images/industries/industry_carpenter.png"
                            on_view_spec=move || active_spec.set(Some(IndustrySpec {
                                title: "Carpenters".to_string(),
                                booster: "Bespoke Joinery // Kitchen Fitting".to_string(),
                                details: vec![
                                    "High-Res Transformation Galleries".to_string(),
                                    "High-Ticket Renovation SEO".to_string(),
                                    "Material-Specific Keyword Clusters".to_string(),
                                    "Interactive Project Showcases".to_string(),
                                ]
                            }))
                        />
                        <IndustryCard
                            title="Bricklayers"
                            visibility="Extensions // Repointing // Walls"
                            description="Structural focus with case study layouts. Ranking for domestic and commercial masonry projects."
                            image="/images/industries/industry_bricklayer.png"
                            on_view_spec=move || active_spec.set(Some(IndustrySpec {
                                title: "Bricklayers".to_string(),
                                booster: "Extensions // Repointing // Walls".to_string(),
                                details: vec![
                                    "Before/After Slider Integration".to_string(),
                                    "Local Extension Planning Keywords".to_string(),
                                    "Structural Integrity Trust Signals".to_string(),
                                    "Heritage Restoration Galleries".to_string(),
                                ]
                            }))
                        />
                        <IndustryCard
                            title="Roofers"
                            visibility="Flat Roofing // Guttering // Re-Roofs"
                            description="Trust-building architecture with guarantee highlighting. Dominating local storm damage searches."
                            image="/images/industries/industry_roofer.png"
                            on_view_spec=move || active_spec.set(Some(IndustrySpec {
                                title: "Roofers".to_string(),
                                booster: "Flat Roofing // Guttering // Re-Roofs".to_string(),
                                details: vec![
                                    "Emergency Storm Damage Funnels".to_string(),
                                    "Drone Survey Video Integration".to_string(),
                                    "Long-Term Guarantee Badges".to_string(),
                                    "Seasonal Gutter Maintenance Leads".to_string(),
                                ]
                            }))
                        />
                        <IndustryCard
                            title="Locksmiths"
                            visibility="Emergency Entry // Security Upgrades"
                            description="The ultimate speed-rank setup. Mobile-first design for customers locked out and in a hurry."
                            image="/images/industries/industry_locksmith.png"
                            on_view_spec=move || active_spec.set(Some(IndustrySpec {
                                title: "Locksmiths".to_string(),
                                booster: "Emergency Entry // Security Upgrades".to_string(),
                                details: vec![
                                    "Click-to-Call Primary UI".to_string(),
                                    "Neighborhood Watch Validation".to_string(),
                                    "24/7 Availability Schema".to_string(),
                                    "Rapid Location Detection Logic".to_string(),
                                ]
                            }))
                        />
                        <IndustryCard
                            title="Landscapers"
                            visibility="Porcelain Patios // Garden Design"
                            description="Focus on high-ticket landscape transformations and seasonal builds."
                            image="/images/industries/industry_landscaper.png"
                            on_view_spec=move || active_spec.set(Some(IndustrySpec {
                                title: "Landscapers".to_string(),
                                booster: "Porcelain Patios // Garden Design".to_string(),
                                details: vec![
                                    "Project Cost Calculator Integration".to_string(),
                                    "Season-Led Content Management".to_string(),
                                    "Portfolio Transformation System".to_string(),
                                    "Planning Permission Lead Gen".to_string(),
                                ]
                            }))
                        />
                        <IndustryCard
                            title="Tilers"
                            visibility="Wet Rooms // Large Format Porcelain"
                            description="Precision-led visibility for luxury bathroom and floor upgrades."
                            image="/images/industries/industry_tiler.png"
                            on_view_spec=move || active_spec.set(Some(IndustrySpec {
                                title: "Tilers".to_string(),
                                booster: "Wet Rooms // Large Format Porcelain".to_string(),
                                details: vec![
                                    "Precision Craftmanship Galleries".to_string(),
                                    "Wet Room Specialist Keywords".to_string(),
                                    "Instant Tile Estimation Forms".to_string(),
                                    "Material-Expert Trust Markers".to_string(),
                                ]
                            }))
                        />
                        <IndustryCard
                            title="Security"
                            visibility="Smart CCTV // Intruder Alarms"
                            description="Dominating the tech-led security market for domestic and commercial."
                            image="/images/industries/industry_security.png"
                            on_view_spec=move || active_spec.set(Some(IndustrySpec {
                                title: "Security".to_string(),
                                booster: "Smart CCTV // Intruder Alarms".to_string(),
                                details: vec![
                                    "Trust & Certification Display".to_string(),
                                    "Smart Home Integration SEO".to_string(),
                                    "Emergency Response Funnels".to_string(),
                                    "Brand-Partner Visibility Prep".to_string(),
                                ]
                            }))
                        />
                    </div>
                </div>
            </section>

            // CTA
            <section class="bg-brand py-20 px-4 text-center">
                <div class="max-w-3xl mx-auto">
                    <h2 class="text-4xl font-black text-void mb-6 tracking-tight">"READY TO DOMINATE YOUR TRADE?"</h2>
                    <p class="text-void/80 text-xl font-medium mb-10">"Stop relying on word of mouth. Build a predictable lead-generation machine."</p>
                    <a href="/pricing" class="btn bg-void text-white hover:bg-void-surface btn-lg">"INITIALIZE MY WEBSITE"</a>
                </div>
            </section>
        </div>
    }
}

#[component]
fn IndustryCard<F>(
    title: &'static str,
    visibility: &'static str,
    description: &'static str,
    image: &'static str,
    on_view_spec: F,
) -> impl IntoView
where
    F: Fn() + Clone + 'static,
{
    let on_click = on_view_spec.clone();
    view! {
        <div class="card-deep group border-void-highlight/30 hover:border-brand/40 flex flex-col h-full bg-void-surface/30 transition-all duration-500 hover:-translate-y-2">
            <div class="relative h-48 overflow-hidden rounded-lg mb-6">
                <img
                    src=image
                    alt=format!("{} services", title)
                    class="w-full h-full object-cover group-hover:scale-110 transition duration-700 opacity-60 group-hover:opacity-80 object-center"
                />
                <div class="absolute inset-0 bg-gradient-to-t from-void to-transparent opacity-60"></div>
                <div class="absolute bottom-4 left-4">
                    <h3 class="text-2xl font-bold text-white tracking-tight">{title}</h3>
                </div>
            </div>

            <div class="flex-grow">
                <div class="text-brand font-mono text-[10px] tracking-widest uppercase mb-3 px-2 py-1 bg-brand/5 border border-brand/10 inline-block rounded">
                    {visibility}
                </div>
                <p class="text-gray-400 text-sm leading-relaxed">{description}</p>
            </div>

            <div class="mt-6 pt-6 border-t border-void-highlight/30">
                <button
                    on:click=move |_| on_click()
                    class="text-white text-xs font-bold font-mono tracking-widest flex items-center gap-2 group/link hover:text-brand transition"
                >
                    "VIEW SPEC"
                    <svg class="w-3 h-3 group-hover/link:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"/></svg>
                </button>
            </div>
        </div>
    }
}

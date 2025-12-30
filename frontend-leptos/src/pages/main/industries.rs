use crate::components::seo::SeoHead;
use leptos::prelude::*;
use shared::PageMetadata;

#[derive(Clone, Default)]
struct IndustrySpec {
    title: String,
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

                <div class="max-w-6xl mx-auto grid md:grid-cols-1 gap-12 items-center relative z-10 text-center">
                    <div class="animate-fade-in mx-auto">
                        <h1 class="text-5xl md:text-7xl font-heading font-black mb-6 tracking-tighter leading-[0.9]">
                            "GROW YOUR" <br/>
                            <span class="text-brand">"TRADESMAN BUSINESS IN 2026."</span>
                        </h1>
                        <p class="text-xl text-gray-400 max-w-xl font-light mb-8 mx-auto">
                            "Most agency sites are generic. We build <span class='text-white font-semibold'>Visibility Engines</span> tailored to the specific jobs that keep your business profitable."
                        </p>
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
                    // Header Removed


                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                        <IndustryCard
                            title="Electricians"
                            description="Reliable electrical services for domestic and commercial projects. From rewiring to safety checks."
                            image="/images/industries/industry_electrician.png"
                            on_view_spec=move || active_spec.set(Some(IndustrySpec {
                                title: "Electricians".to_string(),
                                details: vec![
                                    "Residential Rewiring".to_string(),
                                    "Fuse Board Upgrades".to_string(),
                                    "Testing & Inspection".to_string(),
                                    "Lighting Installations".to_string(),
                                ]
                            }))
                        />
                        <IndustryCard
                            title="Plumbers"
                            description="Professional plumbing repairs, maintenance, and installations for kitchens and bathrooms."
                            image="/images/industries/industry_plumber.png"
                            on_view_spec=move || active_spec.set(Some(IndustrySpec {
                                title: "Plumbers".to_string(),
                                details: vec![
                                    "Leak Detections & Repairs".to_string(),
                                    "Bathroom Installations".to_string(),
                                    "Pipework Maintenance".to_string(),
                                    "Emergency Call-Outs".to_string(),
                                ]
                            }))
                        />
                        <IndustryCard
                            title="Carpenters"
                            description="Bespoke carpentry, joinery, and wood construction services for your home."
                            image="/images/industries/industry_carpenter.png"
                            on_view_spec=move || active_spec.set(Some(IndustrySpec {
                                title: "Carpenters".to_string(),
                                details: vec![
                                    "Custom Furniture".to_string(),
                                    "Door & Window Fitting".to_string(),
                                    "Flooring Installation".to_string(),
                                    "Decking & Fencing".to_string(),
                                ]
                            }))
                        />
                        <IndustryCard
                            title="Bricklayers"
                            description="Expert bricklaying for new builds, extensions, garden walls, and structural repairs."
                            image="/images/industries/industry_bricklayer.png"
                            on_view_spec=move || active_spec.set(Some(IndustrySpec {
                                title: "Bricklayers".to_string(),
                                details: vec![
                                    "Home Extensions".to_string(),
                                    "Garden Walls".to_string(),
                                    "Repointing Services".to_string(),
                                    "Structural Alterations".to_string(),
                                ]
                            }))
                        />
                        <IndustryCard
                            title="Roofers"
                            description="Complete roofing solutions, from minor repairs to full replacements and guttering."
                            image="/images/industries/industry_roofer.png"
                            on_view_spec=move || active_spec.set(Some(IndustrySpec {
                                title: "Roofers".to_string(),
                                details: vec![
                                    "Slating & Tiling".to_string(),
                                    "Flat Roof Systems".to_string(),
                                    "Chimney Repairs".to_string(),
                                    "Guttering & Fascias".to_string(),
                                ]
                            }))
                        />
                        <IndustryCard
                            title="Locksmiths"
                            description="Fast and professional lock services, security solutions, and emergency entry."
                            image="/images/industries/industry_locksmith.png"
                            on_view_spec=move || active_spec.set(Some(IndustrySpec {
                                title: "Locksmiths".to_string(),
                                details: vec![
                                    "24/7 Emergency Entry".to_string(),
                                    "Lock Replacements".to_string(),
                                    "UPVC Door Repairs".to_string(),
                                    "Security Upgrades".to_string(),
                                ]
                            }))
                        />
                        <IndustryCard
                            title="Landscapers"
                            description="Garden transformations, paving, decking, and comprehensive grounds maintenance."
                            image="/images/industries/industry_landscaper.png"
                            on_view_spec=move || active_spec.set(Some(IndustrySpec {
                                title: "Landscapers".to_string(),
                                details: vec![
                                    "Garden Design".to_string(),
                                    "Patios & Paving".to_string(),
                                    "Turfing & Planting".to_string(),
                                    "Fencing Solutions".to_string(),
                                ]
                            }))
                        />
                        <IndustryCard
                            title="Tilers"
                            description="High-quality tiling for walls, floors, wet rooms, and kitchen splashbacks."
                            image="/images/industries/industry_tiler.png"
                            on_view_spec=move || active_spec.set(Some(IndustrySpec {
                                title: "Tilers".to_string(),
                                details: vec![
                                    "Ceramic & Porcelain".to_string(),
                                    "Natural Stone".to_string(),
                                    "Wet Room Tiling".to_string(),
                                    "Floor Levelling".to_string(),
                                ]
                            }))
                        />
                        <IndustryCard
                            title="Security"
                            description="Advanced security system installations including CCTV and alarms for peace of mind."
                            image="/images/industries/industry_security.png"
                            on_view_spec=move || active_spec.set(Some(IndustrySpec {
                                title: "Security".to_string(),
                                details: vec![
                                    "CCTV Installation".to_string(),
                                    "Intruder Alarms".to_string(),
                                    "Access Control".to_string(),
                                    "Smart Home Security".to_string(),
                                ]
                            }))
                        />
                    </div>
                </div>
            </section>


        </div>
    }
}

#[component]
fn IndustryCard<F>(
    title: &'static str,
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

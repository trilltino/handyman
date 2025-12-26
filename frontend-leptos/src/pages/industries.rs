use crate::components::seo::SeoHead;
use leptos::prelude::*;
use shared::PageMetadata;

#[component]
pub fn Industries() -> impl IntoView {
    view! {
        <SeoHead metadata=PageMetadata {
            title: "Websites for Tradesmen | Electricians, Plumbers & More | XF Tradesmen".to_string(),
            description: "Specialized website designs for every trade. From electricians to roofers, get a site that ranks for your specific industry keywords.".to_string(),
            canonical_url: Some("https://xftradesman.com/industries".to_string()),
            og_image: None,
        }/>

        <section class="hero bg-deep-black text-white py-20">
            <div class="container mx-auto px-4 text-center">
                <h1 class="text-5xl font-bold mb-6">
                    <span class="text-white">"Websites for "</span>
                    <span class="text-deep-red">"Tradesmen"</span>
                </h1>
                <p class="text-xl text-gray-300 max-w-2xl mx-auto mb-8">
                    "Specialized, high-ranking websites designed for your specific trade. We understand the keywords, schema, and structure your business needs to grow."
                </p>
            </div>
        </section>

        <section class="bg-white py-16">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl font-bold text-deep-black mb-12 text-center">"Industries We Serve"</h2>

                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                    <IndustryCard
                        title="Electricians"
                        description="Emergency callout features, local service pages, and electrical safety schema."
                        image="/industry_electrician.png"
                    />
                    <IndustryCard
                        title="Plumbers"
                        description="Boiler service booking, emergency leak pages, and Gas Safe highlighting."
                        image="/industry_plumber.png"
                    />
                    <IndustryCard
                        title="Carpenters"
                        description="Portfolio galleries for bespoke joinery, wardrobes, and kitchen installations."
                        image="/industry_carpenter.png"
                    />
                    <IndustryCard
                        title="Bricklayers"
                        description="Showcase extensions, garden walls, and repointing work with high-res imagery."
                        image="/industry_bricklayer.png"
                    />
                     <IndustryCard
                        title="Plasterers"
                        description="Highlight smooth finishes, rendering, and dry lining services."
                        image="/industry_plasterer.png"
                    />
                     <IndustryCard
                        title="Roofers"
                        description="Trust-building elements for new roofs, repairs, and flat roofing."
                        image="/industry_roofer.png"
                    />
                     <IndustryCard
                        title="Painters & Decorators"
                        description="Visual portfolios for interior and exterior transformation projects."
                        image="/industry_painter.png"
                    />
                    <IndustryCard
                        title="Gas Engineers"
                        description="Safety certification display, landlord checks, and boiler servicing."
                        image="/industry_gas_engineer.png"
                    />
                    <IndustryCard
                        title="Locksmiths"
                        description="Fast-response design, click-to-call buttons, and emergency entry focus."
                        image="/industry_locksmith.png"
                    />
                    <IndustryCard
                        title="HVAC Technicians"
                        description="Seasonal service promotion for heating and cooling systems."
                        image="/industry_hvac.png"
                    />
                </div>
            </div>
        </section>

        <section class="cta bg-deep-red text-white py-20">
            <div class="container mx-auto px-4 text-center">
                <h2 class="text-4xl font-bold mb-6">"Ready to Dominate Your Local Market?"</h2>
                <p class="text-xl mb-8">"Get a website that works as hard as you do."</p>
                <a href="/contact" class="inline-block bg-white text-deep-red font-bold py-3 px-8 rounded hover:bg-gray-100 transition">
                    "Get Started"
                </a>
            </div>
        </section>
    }
}

#[component]
fn IndustryCard(
    title: &'static str,
    description: &'static str,
    image: &'static str,
) -> impl IntoView {
    view! {
        <div class="card overflow-hidden border border-gray-100 rounded-lg shadow-sm hover:shadow-xl hover:border-deep-red transition duration-300 group bg-white">
            <div class="relative h-48 overflow-hidden">
                <img
                    src=image
                    alt=format!("{} services", title)
                    class="w-full h-full object-cover group-hover:scale-110 transition duration-500"
                />
                <div class="absolute inset-0 bg-gradient-to-t from-black/60 to-transparent"></div>
                <h3 class="absolute bottom-4 left-4 text-2xl font-bold text-white">{title}</h3>
            </div>
            <div class="p-6">
                <p class="text-gray-600">{description}</p>
            </div>
        </div>
    }
}

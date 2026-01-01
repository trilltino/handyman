use leptos::prelude::*;

#[component]
pub fn HandymanTestimonials() -> impl IntoView {
    view! {
        <div class="bg-gray-50 min-h-screen pb-20">
            // -- Hero Section --
            <div class="relative bg-gradient-to-r from-blue-900 to-blue-800 text-white pt-32 pb-48 px-6 text-center overflow-hidden">
                <div class="relative z-10 max-w-3xl mx-auto">
                    <h1 class="text-4xl md:text-5xl font-bold mb-8 font-heading tracking-tight">"Testimonials"</h1>
                    <p class="text-blue-100 text-lg md:text-xl max-w-2xl mx-auto leading-relaxed">"Read what our satisfied customers in Coventry have to say about our handyman services."</p>
                </div>


            </div>

            // -- Masonry Grid Content --
            <div class="max-w-7xl mx-auto px-6 -mt-10 relative z-20">
                <div class="columns-1 md:columns-2 lg:columns-3 gap-10 space-y-10">

                    // -- Highlight Review Card --
                    <div class="break-inside-avoid bg-white rounded-xl shadow-lg p-8 border-t-4 border-blue-600">
                         <div class="flex justify-between items-start mb-4">
                            <div>
                                <h3 class="font-bold text-gray-900 text-lg">"Absolutely Outstanding"</h3>
                            </div>
                            <div class="flex items-center gap-1 bg-green-50 px-2 py-1 rounded text-xs font-bold text-green-700 border border-green-100">
                                <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path></svg>
                                "Verified"
                            </div>
                         </div>
                        <div class="flex text-yellow-500 mb-4 text-sm">
                            {(0..5).map(|_| view! { <svg class="w-5 h-5 fill-current" viewBox="0 0 24 24"><path d="M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z"/></svg> }).collect::<Vec<_>>()}
                        </div>
                        <p class="text-gray-600 italic mb-6">
                            "I cannot recommend XFTradesmen enough. From the initial quote to the final cleanup, everything was handled with professionalism. The quality of work on my new kitchen cabinets is superb."
                        </p>
                         <div class="flex items-center gap-3 pt-4 border-t border-gray-100">
                            <div class="w-10 h-10 rounded-full bg-blue-100 flex items-center justify-center text-blue-700 font-bold text-sm">
                                "JD"
                            </div>
                            <div>
                                <div class="font-bold text-gray-900 text-sm">"James Davidson"</div>
                                <div class="text-xs text-gray-400">"Local Guide • 12 reviews"</div>
                            </div>
                            <img src="/images/icons/google-logo.svg" onerror="this.style.display='none'" class="w-5 h-5 ml-auto opacity-60" alt="Google" />
                        </div>
                    </div>

                    // -- Standard Reviews --
                    <ReviewCard
                        name="Sarah Jenkins"
                        location="Earlsdon"
                        text="Fixed my leaking tap in 20 minutes. Very polite and tidy."
                        initials="SJ"
                    />
                     <ReviewCard
                        name="Mike Ross"
                        location="ReviewCentre"
                        text="Top notch for flat pack assembly. I usually hate doing it myself, so this was a lifesaver. Good price too."
                        initials="MR"
                    />
                    <ReviewCard
                        name="Emily Blunt"
                        location="Kenilworth"
                        text="We hired them for a full day of odd jobs - hanging pictures, fixing a door handle, and resealing the bath. Got through everything on the list!"
                        initials="EB"
                    />
                    <ReviewCard
                        name="David Chen"
                        location="Coventry"
                        text="Arrived on time for an emergency call out. The door lock was jammed and he managed to fix it without damaging the door. Very impressed."
                        initials="DC"
                    />
                    <ReviewCard
                        name="Patricia Wilson"
                        location="Finham"
                        text="A polite young man who knows his trade. Fixed my garden fence after the storm. It's solid as a rock now."
                        initials="PW"
                    />
                     <ReviewCard
                        name="Gary Thompson"
                        location="Wholey"
                        text="Great communication. Texted me when he was on his way. Did a clean job replacing the light fixtures."
                        initials="GT"
                    />
                    <ReviewCard
                        name="Lisa Kudrow"
                        location="Tile Hill"
                        text="Very happy with the painting service. The lines are crisp and they covered all the furniture properly."
                        initials="LK"
                    />
                     <ReviewCard
                        name="Tom Hardy"
                        location="Binley"
                        text="Honest pricing. No hidden fees at the end. Will definitely use again for the next project."
                        initials="TH"
                    />
                    <ReviewCard
                        name="Rebecca Jones"
                        location="Stoke"
                        text="Friendly, efficient, and did a great job fixing our garden gate. Highly recommended!"
                        initials="RJ"
                    />
                    <ReviewCard
                        name="Kevin Smith"
                        location="Wyken"
                        text="I had a list of small jobs that needed doing for months. He came in and sorted them all in one afternoon. Brilliant service."
                        initials="KS"
                    />
                    <ReviewCard
                        name="Amanda White"
                        location="Coundon"
                        text="Very professional. Cleaned up after himself which is a rarity these days. The new shelf looks perfect."
                        initials="AW"
                    />
                    <ReviewCard
                        name="Steve Brown"
                        location="Radford"
                        text="Quick response to my enquiry. Arrived on time and fixed the leak under the sink. No fuss."
                        initials="SB"
                    />
                    <ReviewCard
                        name="Natalie Green"
                        location="Holbrooks"
                        text="Explained exactly what needed doing and how much it would cost before starting. Very trustworthy."
                        initials="NG"
                    />
                    <ReviewCard
                        name="Paul Wilson"
                        location="Cheylesmore"
                        text="Replaced all the door handles in my house. Quick, tidy, and a great price."
                        initials="PW"
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
fn ReviewCard(
    name: &'static str,
    location: &'static str,
    text: &'static str,
    initials: &'static str,
) -> impl IntoView {
    view! {
        <div class="break-inside-avoid bg-white rounded-xl shadow-sm p-6 hover:shadow-md transition duration-300 border border-gray-100">
             <div class="flex justify-between items-start mb-3">
                 <div class="flex text-yellow-400 text-xs">
                    {(0..5).map(|_| view! { <svg class="w-4 h-4 fill-current" viewBox="0 0 24 24"><path d="M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z"/></svg> }).collect::<Vec<_>>()}
                </div>
                <div class="text-[10px] font-bold text-gray-300 uppercase tracking-wide">"Verified"</div>
             </div>

             <p class="text-gray-600 text-sm leading-relaxed mb-4">
                "\"" {text} "\""
            </p>

            <div class="flex items-center gap-3">
                <div class="w-8 h-8 rounded-full bg-gray-100 flex items-center justify-center text-gray-600 font-bold text-xs ring-2 ring-white">
                    {initials}
                </div>
                <div>
                    <div class="font-bold text-gray-900 text-xs">{name}</div>
                    <div class="text-[10px] text-gray-400 font-medium uppercase">{location}</div>
                </div>
                // Placeholder for Google 'G' icon - using text/svg backup in reality, or just an SVG path
                 <div class="ml-auto">
                    <svg class="w-4 h-4 text-gray-300" viewBox="0 0 24 24" fill="currentColor"><path d="M12.545 10.239v3.821h5.445c-0.712 2.315-2.647 3.972-5.445 3.972-3.332 0-6.033-2.53-6.033-5.652s2.701-5.652 6.033-5.652c1.498 0 2.866 0.549 3.921 1.453l2.814-2.814c-1.79-1.677-4.184-2.702-6.735-2.702-5.522 0-10 4.478-10 10s4.478 10 10 10c8.396 0 10.249-7.784 9.42-11.428h-9.42z"/></svg>
                 </div>
            </div>
        </div>
    }
}

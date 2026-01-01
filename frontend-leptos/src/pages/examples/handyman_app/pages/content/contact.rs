use leptos::prelude::*;

#[component]
pub fn HandymanContact() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50 font-sans text-gray-800">
            // -- Header / Banner Section --
            <div class="bg-blue-950 text-white py-12 px-6 text-center relative overflow-hidden">
                 // Decorative background element (optional)
                <div class="absolute top-0 left-0 w-full h-full opacity-10 pointer-events-none bg-[radial-gradient(ellipse_at_top,_var(--tw-gradient-stops))] from-blue-400 via-blue-950 to-blue-950"></div>

                <h1 class="relative z-10 text-4xl md:text-5xl font-bold font-heading mb-4 tracking-tight">"Contact Us"</h1>
                <p class="relative z-10 text-xl text-blue-200 max-w-2xl mx-auto">"Get in touch for a free quote or to discuss your home improvement needs."</p>
            </div>

            // -- Main Content Split --
            <div class="max-w-7xl mx-auto px-4 py-16 lg:py-24 grid lg:grid-cols-2 gap-12 lg:gap-20 items-start">

                // -- Left Column: Visuals & Info --
                <div class="space-y-12">
                     // Main Image
                    <div class="rounded-2xl overflow-hidden shadow-2xl transform transition hover:scale-[1.01] duration-500 relative aspect-[4/3]">
                         <img
                            src="/images/mockups/uk_handyman_van.png"
                            alt="Our Team"
                            class="w-full h-full object-cover"
                        />
                         // Overlay with Quick Contact
                        <div class="absolute bottom-0 left-0 w-full bg-gradient-to-t from-blue-950/90 to-transparent p-8 text-white pt-24">
                            <h3 class="text-2xl font-bold mb-2">"Ready to start?"</h3>
                            <p class="text-blue-100">"Our team is standing by to help with your project."</p>
                        </div>
                    </div>

                    // Contact Details Block - 3 Card Layout
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                        // Phone Card
                        <div class="bg-blue-50 p-6 rounded-2xl flex flex-col items-center text-center space-y-3 hover:shadow-md transition">
                            <div class="w-12 h-12 rounded-full border-2 border-blue-600 flex items-center justify-center text-blue-600 mb-2">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z"></path></svg>
                            </div>
                            <h4 class="text-lg font-bold text-blue-950">"Phone"</h4>
                            <a href="tel:02476123456" class="text-gray-600 font-medium hover:text-blue-700 decoration-blue-300 decoration-2 underline-offset-2 transition">
                                "(024) 7612-3456"
                            </a>
                        </div>

                         // Location Card
                        <div class="bg-blue-50 p-6 rounded-2xl flex flex-col items-center text-center space-y-3 hover:shadow-md transition">
                             <div class="w-12 h-12 rounded-full border-2 border-blue-600 flex items-center justify-center text-blue-600 mb-2">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"></path><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"></path></svg>
                            </div>
                            <h4 class="text-lg font-bold text-blue-950">"Location"</h4>
                            <p class="text-gray-600 font-medium">
                                "Coventry, UK"
                            </p>
                        </div>

                        // Email Card
                        <div class="bg-blue-50 p-6 rounded-2xl flex flex-col items-center text-center space-y-3 hover:shadow-md transition">
                             <div class="w-12 h-12 rounded-full border-2 border-blue-600 flex items-center justify-center text-blue-600 mb-2">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"></path></svg>
                            </div>
                            <h4 class="text-lg font-bold text-blue-950">"Email"</h4>
                            <a href="mailto:info@handyman-coventry.co.uk" class="text-gray-600 font-medium hover:text-blue-700 decoration-blue-300 decoration-2 underline-offset-2 transition">
                                "Click To Email"
                            </a>
                        </div>
                    </div>
                </div>

                // -- Right Column: Contact Form --
                <div class="bg-white rounded-2xl shadow-xl p-8 md:p-12 border border-gray-100">
                    <div class="mb-10">
                        <h2 class="text-3xl font-bold text-blue-950 mb-4">"Send us a Message"</h2>
                        <p class="text-gray-600">"Fill out the form below and we'll get back to you as soon as possible with a free estimate."</p>
                    </div>

                    <form class="space-y-6" onsubmit="event.preventDefault();">
                        <div class="grid md:grid-cols-2 gap-6">
                            <div class="space-y-2">
                                <label for="name" class="text-sm font-semibold text-gray-700">"Full Name"</label>
                                <input type="text" id="name" placeholder="John Doe"
                                    class="w-full px-4 py-3 rounded-lg border border-gray-200 focus:border-blue-500 focus:ring-2 focus:ring-blue-200 outline-none transition bg-gray-50 focus:bg-white" />
                            </div>
                             <div class="space-y-2">
                                <label for="phone" class="text-sm font-semibold text-gray-700">"Phone Number"</label>
                                <input type="tel" id="phone" placeholder="07123 456789"
                                    class="w-full px-4 py-3 rounded-lg border border-gray-200 focus:border-blue-500 focus:ring-2 focus:ring-blue-200 outline-none transition bg-gray-50 focus:bg-white" />
                            </div>
                        </div>

                        <div class="grid md:grid-cols-2 gap-6">
                             <div class="space-y-2">
                                <label for="email" class="text-sm font-semibold text-gray-700">"Email Address"</label>
                                <input type="email" id="email" placeholder="john@example.com"
                                    class="w-full px-4 py-3 rounded-lg border border-gray-200 focus:border-blue-500 focus:ring-2 focus:ring-blue-200 outline-none transition bg-gray-50 focus:bg-white" />
                            </div>
                            <div class="space-y-2">
                                <label for="location" class="text-sm font-semibold text-gray-700">"Location"</label>
                                <input type="text" id="location" placeholder="e.g. Earlsdon"
                                    class="w-full px-4 py-3 rounded-lg border border-gray-200 focus:border-blue-500 focus:ring-2 focus:ring-blue-200 outline-none transition bg-gray-50 focus:bg-white" />
                            </div>
                        </div>

                         <div class="space-y-2">
                            <label for="discovery" class="text-sm font-semibold text-gray-700">"How did you hear about us?"</label>
                            <div class="relative">
                                <select id="discovery" class="w-full px-4 py-3 rounded-lg border border-gray-200 focus:border-blue-500 focus:ring-2 focus:ring-blue-200 outline-none transition bg-gray-50 focus:bg-white appearance-none cursor-pointer">
                                    <option value="" disabled selected>"Select an option"</option>
                                    <option value="google">"Google Search"</option>
                                    <option value="social">"Social Media"</option>
                                    <option value="recommendation">"Friend Recommendation"</option>
                                    <option value="leaflet">"Leaflet / Flyer"</option>
                                    <option value="other">"Other"</option>
                                </select>
                                <div class="absolute inset-y-0 right-0 flex items-center px-4 pointer-events-none text-gray-500">
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path></svg>
                                </div>
                            </div>
                        </div>

                        <div class="space-y-2">
                            <label for="message" class="text-sm font-semibold text-gray-700">"Project Details"</label>
                            <textarea id="message" rows="4" placeholder="Tell us about what needs fixing..."
                                class="w-full px-4 py-3 rounded-lg border border-gray-200 focus:border-blue-500 focus:ring-2 focus:ring-blue-200 outline-none transition bg-gray-50 focus:bg-white resize-none"></textarea>
                        </div>

                        <div class="space-y-2">
                            <span class="text-sm font-semibold text-gray-700 block">"Upload Photos (Optional)"</span>
                            <div class="border-2 border-dashed border-gray-300 rounded-lg p-8 text-center hover:bg-gray-50 transition cursor-pointer group">
                                <div class="w-12 h-12 bg-blue-50 text-blue-500 rounded-full flex items-center justify-center mx-auto mb-3 group-hover:scale-110 transition">
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>
                                </div>
                                <span class="text-blue-600 font-medium group-hover:underline">"Click to upload"</span>
                                <span class="text-gray-500">" or drag and drop"</span>
                                <p class="text-xs text-gray-400 mt-1">"SVG, PNG, JPG or GIF (max. 800x400px)"</p>
                                <input type="file" class="hidden" />
                            </div>
                        </div>

                        <button type="submit" class="w-full bg-blue-600 hover:bg-blue-700 text-white font-bold py-4 rounded-lg shadow-lg hover:shadow-xl transform transition hover:-translate-y-0.5 active:translate-y-0 duration-200 flex items-center justify-center gap-2">
                            <span>"Get Free Quote"</span>
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"></path></svg>
                        </button>
                    </form>
                </div>
            </div>

            // -- Footer / Accreditation Strip (Optional addition based on muse) --
            <div class="bg-white border-t border-gray-200 py-12">
                <div class="max-w-7xl mx-auto px-6 text-center">
                    <p class="text-gray-500 font-medium mb-6 uppercase tracking-widest text-sm">"Trusted by homeowners across Coventry"</p>
                     <div class="flex flex-wrap justify-center gap-8 md:gap-16 opacity-60 grayscale hover:grayscale-0 transition duration-500">
                        // Placeholders for trust logos - text for now
                         <span class="text-xl font-bold font-serif text-gray-400">"ReviewCentre"</span>
                         <span class="text-xl font-bold font-serif text-gray-400">"TrustPilot"</span>
                         <span class="text-xl font-bold font-serif text-gray-400">"Google Reviews"</span>
                         <span class="text-xl font-bold font-serif text-gray-400">"Checkatrade"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

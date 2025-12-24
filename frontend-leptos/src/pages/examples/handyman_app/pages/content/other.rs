use leptos::prelude::*;

#[component]
pub fn HandymanAbout() -> impl IntoView {
    view! {
        <div class="py-24 px-6 text-center">
            <h1 class="text-4xl font-bold text-blue-900 mb-4">"About Us"</h1>
            <p class="text-gray-600">"We are a team of dedicated trade professionals based in Coventry."</p>
        </div>
    }
}

#[component]
pub fn HandymanContact() -> impl IntoView {
    view! {
        <div class="py-24 px-6 text-center bg-gray-50">
            <h1 class="text-4xl font-bold text-blue-900 mb-4">"Contact Us"</h1>
             <p class="text-gray-600 mb-8">"Get a quote today."</p>
             <a href="tel:07833263486" class="inline-block bg-yellow-500 text-blue-900 font-bold px-8 py-3 rounded">"Call Now"</a>
        </div>
    }
}

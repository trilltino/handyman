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

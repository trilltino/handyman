//! Legal pages: Privacy Policy and Terms of Service.

use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn PrivacyPolicy() -> impl IntoView {
    view! {
        <Title text="Privacy Policy | Coventry Handyman"/>

        <div class="min-h-screen bg-slate-50 py-12 px-6">
            <article class="max-w-3xl mx-auto bg-white rounded-xl p-8 shadow-sm prose prose-slate">
                <h1>"Privacy Policy"</h1>
                <p class="text-slate-500">"Last updated: December 23, 2024"</p>

                <h2>"Information We Collect"</h2>
                <p>"When you use our services or contact us, we may collect:"</p>
                <ul>
                    <li>"Your name and contact details (phone, email, address)"</li>
                    <li>"Information about your property and the services you require"</li>
                    <li>"Payment information for billing purposes"</li>
                </ul>

                <h2>"How We Use Your Information"</h2>
                <p>"We use your information to:"</p>
                <ul>
                    <li>"Provide handyman services as requested"</li>
                    <li>"Contact you about bookings and appointments"</li>
                    <li>"Send invoices and process payments"</li>
                    <li>"Improve our services"</li>
                </ul>

                <h2>"Data Protection"</h2>
                <p>"We take data protection seriously. Your personal information is stored securely and is never sold to third parties. We comply with UK GDPR requirements."</p>

                <h2>"Your Rights"</h2>
                <p>"You have the right to:"</p>
                <ul>
                    <li>"Request access to your personal data"</li>
                    <li>"Request correction of inaccurate data"</li>
                    <li>"Request deletion of your data"</li>
                    <li>"Withdraw consent at any time"</li>
                </ul>

                <h2>"Contact Us"</h2>
                <p>"For any privacy-related queries, please contact us at:"</p>
                <p>
                    <strong>"Email: "</strong>"hello@xftradesman.com"<br/>
                    <strong>"Phone: "</strong>"07833 263486"
                </p>
            </article>
        </div>
    }
}

#[component]
pub fn TermsOfService() -> impl IntoView {
    view! {
        <Title text="Terms of Service | Coventry Handyman"/>

        <div class="min-h-screen bg-slate-50 py-12 px-6">
            <article class="max-w-3xl mx-auto bg-white rounded-xl p-8 shadow-sm prose prose-slate">
                <h1>"Terms of Service"</h1>
                <p class="text-slate-500">"Last updated: December 23, 2024"</p>

                <h2>"1. Services"</h2>
                <p>"Coventry Handyman provides general handyman services including plumbing, electrical work, carpentry, furniture assembly, and general repairs. All work is carried out by qualified professionals."</p>

                <h2>"2. Bookings"</h2>
                <p>"By booking our services, you agree to:"</p>
                <ul>
                    <li>"Provide accurate information about the work required"</li>
                    <li>"Ensure safe access to your property"</li>
                    <li>"Be present or arrange access for scheduled appointments"</li>
                    <li>"Pay for services as agreed"</li>
                </ul>

                <h2>"3. Pricing"</h2>
                <p>"All prices quoted are estimates based on the information provided. Final pricing may vary based on actual work required. We will always discuss any changes with you before proceeding."</p>

                <h2>"4. Cancellations"</h2>
                <p>"Please provide at least 24 hours notice for cancellations. Late cancellations may incur a call-out fee."</p>

                <h2>"5. Guarantee"</h2>
                <p>"We guarantee our workmanship for 12 months from completion. This does not cover materials unless explicitly stated, or issues arising from misuse."</p>

                <h2>"6. Liability"</h2>
                <p>"We carry £2 million public liability insurance. However, we are not liable for pre-existing issues or problems not related to our work."</p>

                <h2>"7. Contact"</h2>
                <p>"For any questions about these terms, please contact us."</p>
            </article>
        </div>
    }
}

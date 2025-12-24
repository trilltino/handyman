//! Customer Management page for admin.
//!
//! Shows customer list with search and customer details.

use leptos::prelude::*;
use leptos_meta::Title;

#[derive(Clone, Debug, PartialEq)]
struct Customer {
    id: String,
    name: String,
    email: String,
    phone: String,
    total_jobs: i32,
    total_spent: i32,
    last_job: String,
}

fn get_mock_customers() -> Vec<Customer> {
    vec![
        Customer {
            id: "1".to_string(),
            name: "John Smith".to_string(),
            email: "john.smith@email.com".to_string(),
            phone: "07123 456789".to_string(),
            total_jobs: 5,
            total_spent: 485,
            last_job: "Dec 20, 2024".to_string(),
        },
        Customer {
            id: "2".to_string(),
            name: "Sarah Jones".to_string(),
            email: "sarah.j@email.com".to_string(),
            phone: "07987 654321".to_string(),
            total_jobs: 3,
            total_spent: 275,
            last_job: "Dec 18, 2024".to_string(),
        },
        Customer {
            id: "3".to_string(),
            name: "David Williams".to_string(),
            email: "dwilliams@email.com".to_string(),
            phone: "07555 123456".to_string(),
            total_jobs: 8,
            total_spent: 920,
            last_job: "Dec 22, 2024".to_string(),
        },
        Customer {
            id: "4".to_string(),
            name: "Emma Brown".to_string(),
            email: "emma.brown@email.com".to_string(),
            phone: "07444 789012".to_string(),
            total_jobs: 2,
            total_spent: 150,
            last_job: "Nov 15, 2024".to_string(),
        },
    ]
}

#[component]
pub fn AdminCustomers() -> impl IntoView {
    let customers = get_mock_customers();
    let (search, set_search) = signal(String::new());

    let filtered_customers = Memo::new(move |_| {
        let query = search.get().to_lowercase();
        if query.is_empty() {
            customers.clone()
        } else {
            customers
                .iter()
                .filter(|c| {
                    c.name.to_lowercase().contains(&query)
                        || c.email.to_lowercase().contains(&query)
                        || c.phone.contains(&query)
                })
                .cloned()
                .collect()
        }
    });

    view! {
        <Title text="Customers | Admin"/>

        <div class="min-h-screen bg-slate-100">
            // Top bar
            <header class="bg-blue-900 text-white py-4 px-6 flex justify-between items-center">
                <div class="flex items-center gap-3">
                    <div class="w-10 h-10 bg-yellow-500 rounded-lg flex items-center justify-center text-blue-900 font-black text-xl">"H"</div>
                    <div>
                        <div class="font-bold">"Customers"</div>
                        <div class="text-sm text-blue-200">"Manage your customer database"</div>
                    </div>
                </div>
                <a href="/handyman-coventry/admin" class="text-sm text-blue-200 hover:text-white">"Back to Dashboard"</a>
            </header>

            <div class="p-8">
                // Search & Actions
                <div class="flex justify-between items-center mb-6">
                    <div class="relative">
                        <input
                            type="text"
                            placeholder="Search customers..."
                            class="w-80 px-4 py-3 pl-10 rounded-lg border border-slate-300 focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20"
                            prop:value=move || search.get()
                            on:input=move |ev| set_search.set(event_target_value(&ev))
                        />
                        <svg class="w-5 h-5 text-slate-400 absolute left-3 top-1/2 -translate-y-1/2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                        </svg>
                    </div>
                    <button class="px-6 py-3 bg-blue-600 text-white rounded-lg font-bold hover:bg-blue-700 transition flex items-center gap-2">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
                        </svg>
                        "Add Customer"
                    </button>
                </div>

                // Customers Table
                <div class="bg-white rounded-xl shadow-sm overflow-hidden">
                    <table class="w-full">
                        <thead class="bg-slate-50 border-b border-slate-100">
                            <tr>
                                <th class="text-left px-6 py-4 text-sm font-bold text-slate-600">"Customer"</th>
                                <th class="text-left px-6 py-4 text-sm font-bold text-slate-600">"Contact"</th>
                                <th class="text-left px-6 py-4 text-sm font-bold text-slate-600">"Jobs"</th>
                                <th class="text-left px-6 py-4 text-sm font-bold text-slate-600">"Total Spent"</th>
                                <th class="text-left px-6 py-4 text-sm font-bold text-slate-600">"Last Job"</th>
                                <th class="text-left px-6 py-4 text-sm font-bold text-slate-600">"Actions"</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-slate-100">
                            {move || {
                                filtered_customers.get().iter().map(|customer| {
                                    let customer = customer.clone();
                                    view! {
                                        <tr class="hover:bg-slate-50 transition">
                                            <td class="px-6 py-4">
                                                <div class="flex items-center gap-3">
                                                    <div class="w-10 h-10 bg-blue-100 rounded-full flex items-center justify-center text-blue-600 font-bold">
                                                        {customer.name.chars().next().unwrap_or('?').to_string()}
                                                    </div>
                                                    <div class="font-bold text-slate-900">{customer.name.clone()}</div>
                                                </div>
                                            </td>
                                            <td class="px-6 py-4">
                                                <div class="text-sm text-slate-900">{customer.email.clone()}</div>
                                                <div class="text-sm text-slate-500">{customer.phone.clone()}</div>
                                            </td>
                                            <td class="px-6 py-4">
                                                <span class="font-bold text-slate-900">{customer.total_jobs}</span>
                                            </td>
                                            <td class="px-6 py-4">
                                                <span class="font-bold text-green-600">{format!("£{}", customer.total_spent)}</span>
                                            </td>
                                            <td class="px-6 py-4 text-slate-500">{customer.last_job.clone()}</td>
                                            <td class="px-6 py-4">
                                                <div class="flex gap-2">
                                                    <button class="p-2 rounded-lg hover:bg-blue-100 text-blue-600" title="View">
                                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"/>
                                                        </svg>
                                                    </button>
                                                    <button class="p-2 rounded-lg hover:bg-slate-100 text-slate-600" title="Edit">
                                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"/>
                                                        </svg>
                                                    </button>
                                                    <button class="p-2 rounded-lg hover:bg-green-100 text-green-600" title="New Job">
                                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
                                                        </svg>
                                                    </button>
                                                </div>
                                            </td>
                                        </tr>
                                    }
                                }).collect_view()
                            }}
                        </tbody>
                    </table>
                </div>

                // Stats
                <div class="grid grid-cols-3 gap-6 mt-8">
                    <div class="bg-white rounded-xl p-6 shadow-sm">
                        <div class="text-sm text-slate-500">"Total Customers"</div>
                        <div class="text-3xl font-black text-slate-900">"127"</div>
                    </div>
                    <div class="bg-white rounded-xl p-6 shadow-sm">
                        <div class="text-sm text-slate-500">"Active This Month"</div>
                        <div class="text-3xl font-black text-blue-600">"34"</div>
                    </div>
                    <div class="bg-white rounded-xl p-6 shadow-sm">
                        <div class="text-sm text-slate-500">"Average Lifetime Value"</div>
                        <div class="text-3xl font-black text-green-600">"£285"</div>
                    </div>
                </div>
            </div>
        </div>
    }
}

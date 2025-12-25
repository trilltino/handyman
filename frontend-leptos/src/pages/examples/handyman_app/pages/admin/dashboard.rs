//! Admin Dashboard for handyman.
//!
//! Shows overview of jobs, revenue, and quick actions.

use leptos::prelude::*;
use leptos_meta::Title;

/// Mock job data for demonstration
#[derive(Clone, Debug)]
struct Job {
    _id: String,
    customer_name: String,
    location: String,
    service: String,
    time: String,
    status: String,
}

/// Mock stats for dashboard
struct DashboardStats {
    today_jobs: i32,
    today_jobs_completed: i32,
    week_jobs: i32,
    week_revenue: i32,
    _month_revenue: i32,
    pending_quotes: i32,
}

fn get_mock_stats() -> DashboardStats {
    DashboardStats {
        today_jobs: 3,
        today_jobs_completed: 1,
        week_jobs: 12,
        week_revenue: 1245,
        _month_revenue: 4850,
        pending_quotes: 4,
    }
}

fn get_mock_jobs() -> Vec<Job> {
    vec![
        Job {
            _id: "1".to_string(),
            customer_name: "John Smith".to_string(),
            location: "CV6 2AB".to_string(),
            service: "Leaky kitchen tap".to_string(),
            time: "09:00 - 10:30".to_string(),
            status: "confirmed".to_string(),
        },
        Job {
            _id: "2".to_string(),
            customer_name: "Sarah Jones".to_string(),
            location: "CV5 8PQ".to_string(),
            service: "TV wall mount".to_string(),
            time: "11:00 - 12:00".to_string(),
            status: "confirmed".to_string(),
        },
        Job {
            _id: "3".to_string(),
            customer_name: "David Williams".to_string(),
            location: "B92 7QP".to_string(),
            service: "Door repair".to_string(),
            time: "14:00 - 15:30".to_string(),
            status: "pending".to_string(),
        },
    ]
}

#[component]
pub fn AdminDashboard() -> impl IntoView {
    let stats = get_mock_stats();
    let jobs = get_mock_jobs();

    view! {
        <Title text="Dashboard | Coventry Handyman Admin"/>

        <div class="min-h-screen bg-slate-100">
            // Top bar
            <header class="bg-blue-900 text-white py-4 px-6 flex justify-between items-center">
                <div class="flex items-center gap-3">
                    <div class="w-10 h-10 bg-yellow-500 rounded-lg flex items-center justify-center text-blue-900 font-black text-xl">"H"</div>
                    <div>
                        <div class="font-bold">"Admin Dashboard"</div>
                        <div class="text-sm text-blue-200">"Coventry Handyman"</div>
                    </div>
                </div>
                <div class="flex items-center gap-4">
                    <span class="text-sm">"Welcome, Admin"</span>
                    <a href="/handyman-coventry" class="text-sm text-blue-200 hover:text-white">"View Site"</a>
                </div>
            </header>

            <div class="flex">
                // Sidebar
                <aside class="w-64 bg-slate-900 text-slate-300 min-h-screen p-6">
                    <nav class="space-y-2">
                        <a href="/handyman-coventry/admin" class="block px-4 py-3 rounded-lg bg-blue-600 text-white font-medium">
                            "Dashboard"
                        </a>
                        <a href="/handyman-coventry/admin/jobs" class="block px-4 py-3 rounded-lg hover:bg-slate-800 transition">
                            "Jobs"
                        </a>
                        <a href="/handyman-coventry/admin/calendar" class="block px-4 py-3 rounded-lg hover:bg-slate-800 transition">
                            "Calendar"
                        </a>
                        <a href="/handyman-coventry/admin/quotes" class="block px-4 py-3 rounded-lg hover:bg-slate-800 transition">
                            "Quotes"
                        </a>
                        <a href="/handyman-coventry/admin/customers" class="block px-4 py-3 rounded-lg hover:bg-slate-800 transition">
                            "Customers"
                        </a>
                        <a href="/handyman-coventry/admin/analytics" class="block px-4 py-3 rounded-lg hover:bg-slate-800 transition">
                            "Analytics"
                        </a>
                    </nav>
                </aside>

                // Main content
                <main class="flex-1 p-8">
                    // Date header
                    <div class="mb-8 flex justify-between items-center">
                        <div>
                            <h1 class="text-3xl font-bold text-slate-900">"Today's Overview"</h1>
                            <p class="text-slate-500">"Monday, December 23, 2024"</p>
                        </div>
                        <button class="px-6 py-3 bg-blue-600 text-white rounded-lg font-bold hover:bg-blue-700 transition flex items-center gap-2">
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
                            </svg>
                            "Add New Job"
                        </button>
                    </div>

                    // Stats cards
                    <div class="grid grid-cols-4 gap-6 mb-8">
                        <div class="bg-white rounded-xl p-6 shadow-sm">
                            <div class="text-sm font-medium text-slate-500 mb-1">"Today's Jobs"</div>
                            <div class="flex items-end gap-2">
                                <span class="text-4xl font-black text-slate-900">{stats.today_jobs}</span>
                                <span class="text-sm text-slate-500 mb-1">
                                    {format!("{} completed", stats.today_jobs_completed)}
                                </span>
                            </div>
                            <div class="mt-3 h-2 bg-slate-100 rounded-full overflow-hidden">
                                <div
                                    class="h-full bg-blue-500 rounded-full"
                                    style=format!("width: {}%", (stats.today_jobs_completed as f32 / stats.today_jobs as f32 * 100.0) as i32)
                                ></div>
                            </div>
                        </div>

                        <div class="bg-white rounded-xl p-6 shadow-sm">
                            <div class="text-sm font-medium text-slate-500 mb-1">"This Week"</div>
                            <div class="text-4xl font-black text-slate-900">{stats.week_jobs}" jobs"</div>
                            <div class="text-sm text-green-600 mt-2 flex items-center gap-1">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 10l7-7m0 0l7 7m-7-7v18"/>
                                </svg>
                                "20% from last week"
                            </div>
                        </div>

                        <div class="bg-white rounded-xl p-6 shadow-sm">
                            <div class="text-sm font-medium text-slate-500 mb-1">"Week Revenue"</div>
                            <div class="text-4xl font-black text-green-600">
                                {format!("£{}", stats.week_revenue)}
                            </div>
                            <div class="text-sm text-green-600 mt-2 flex items-center gap-1">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 10l7-7m0 0l7 7m-7-7v18"/>
                                </svg>
                                "27% from last week"
                            </div>
                        </div>

                        <div class="bg-white rounded-xl p-6 shadow-sm">
                            <div class="text-sm font-medium text-slate-500 mb-1">"Pending Quotes"</div>
                            <div class="text-4xl font-black text-amber-500">{stats.pending_quotes}</div>
                            <a href="/handyman-coventry/admin/quotes" class="text-sm text-blue-600 mt-2 inline-block hover:underline">
                                "View all quotes"
                            </a>
                        </div>
                    </div>

                    // Today's jobs
                    <div class="bg-white rounded-xl shadow-sm overflow-hidden">
                        <div class="px-6 py-4 border-b border-slate-100 flex justify-between items-center">
                            <h2 class="text-xl font-bold text-slate-900">"Today's Jobs"</h2>
                            <a href="/handyman-coventry/admin/jobs" class="text-sm text-blue-600 hover:underline">"View all"</a>
                        </div>

                        <div class="divide-y divide-slate-100">
                            {jobs.iter().map(|job| {
                                let status_class = if job.status == "confirmed" {
                                    "bg-green-100 text-green-700"
                                } else {
                                    "bg-amber-100 text-amber-700"
                                };

                                view! {
                                    <div class="px-6 py-4 flex items-center justify-between hover:bg-slate-50 transition">
                                        <div class="flex items-center gap-4">
                                            <div class="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center text-blue-600 font-bold">
                                                {job.time[..5].to_string()}
                                            </div>
                                            <div>
                                                <div class="font-bold text-slate-900">
                                                    {job.customer_name.clone()}" - "{job.location.clone()}
                                                </div>
                                                <div class="text-sm text-slate-500">{job.service.clone()}</div>
                                            </div>
                                        </div>
                                        <div class="flex items-center gap-3">
                                            <span class={format!("px-3 py-1 rounded-full text-sm font-medium {}", status_class)}>
                                                {job.status.clone()}
                                            </span>
                                            <div class="flex gap-2">
                                                <button class="p-2 rounded-lg hover:bg-blue-100 text-blue-600 transition" title="Start Job">
                                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"/>
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                                    </svg>
                                                </button>
                                                <button class="p-2 rounded-lg hover:bg-slate-100 text-slate-600 transition" title="Reschedule">
                                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                                                    </svg>
                                                </button>
                                                <button class="p-2 rounded-lg hover:bg-red-100 text-red-600 transition" title="Cancel">
                                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                                                    </svg>
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>

                    // Quick actions
                    <div class="grid grid-cols-4 gap-4 mt-8">
                        <button class="bg-white rounded-xl p-6 shadow-sm text-center hover:shadow-md hover:-translate-y-0.5 transition">
                            <div class="w-12 h-12 mx-auto mb-3 bg-blue-100 rounded-lg flex items-center justify-center">
                                <svg class="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                                </svg>
                            </div>
                            <div class="font-bold text-slate-900">"Add Job"</div>
                        </button>
                        <button class="bg-white rounded-xl p-6 shadow-sm text-center hover:shadow-md hover:-translate-y-0.5 transition">
                            <div class="w-12 h-12 mx-auto mb-3 bg-green-100 rounded-lg flex items-center justify-center">
                                <svg class="w-6 h-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                </svg>
                            </div>
                            <div class="font-bold text-slate-900">"Create Quote"</div>
                        </button>
                        <button class="bg-white rounded-xl p-6 shadow-sm text-center hover:shadow-md hover:-translate-y-0.5 transition">
                            <div class="w-12 h-12 mx-auto mb-3 bg-purple-100 rounded-lg flex items-center justify-center">
                                <svg class="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
                                </svg>
                            </div>
                            <div class="font-bold text-slate-900">"Add Customer"</div>
                        </button>
                        <button class="bg-white rounded-xl p-6 shadow-sm text-center hover:shadow-md hover:-translate-y-0.5 transition">
                            <div class="w-12 h-12 mx-auto mb-3 bg-amber-100 rounded-lg flex items-center justify-center">
                                <svg class="w-6 h-6 text-amber-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
                                </svg>
                            </div>
                            <div class="font-bold text-slate-900">"View Reports"</div>
                        </button>
                    </div>
                </main>
            </div>
        </div>
    }
}

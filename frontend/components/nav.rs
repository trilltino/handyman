//! # Navigation Component
//!
//! ## Purpose
//! Responsive navigation bar displayed on all pages.
//! Shows different menu items based on authentication state.
//!
//! ## Features
//! - Logo and site name
//! - Responsive hamburger menu for mobile
//! - Dynamic menu (Login/Register when logged out, Logout when logged in)
//! - Links to all main pages (Home, Booking, Contact, Experience, Map)
//! - Uses AuthContext to show/hide auth-related links
//!
//! ## Relation to Entire Program
//! - **Rendered By**: App component (visible on every page)
//! - **Uses**: AuthContext for user state, API client for logout

use yew::prelude::*;        // Yew framework
use yew_router::prelude::*; // Routing
use stylist::css;           // CSS-in-Rust styling

use crate::api;             // API client for logout
use crate::contexts::{AuthAction, AuthContext};  // Auth state management
use crate::Route;           // Route enum for navigation links

#[function_component]
pub fn Nav() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let navbar_active = use_state_eq(|| false);

    let toggle_navbar = {
        let navbar_active = navbar_active.clone();
        Callback::from(move |_| {
            navbar_active.set(!*navbar_active);
        })
    };

    let on_logout = {
        let auth = auth.clone();
        Callback::from(move |_| {
            let auth = auth.clone();
            let on_success = Callback::from(move |_| {
                auth.dispatch(AuthAction::Logout);
            });
            let on_error = Callback::from(|err: String| {
                web_sys::console::error_1(&format!("Logout error: {}", err).into());
            });
            api::logout(on_success, on_error);
        })
    };

    let active_class = if *navbar_active { "active" } else { "" };
    let css = css!(r#"
        nav {
            background: linear-gradient(135deg, var(--pure-white) 0%, var(--off-white) 100%);
            padding: 1.2rem 0;
            box-shadow: 0 4px 12px var(--shadow-dark);
            border-bottom: 3px solid var(--primary-red);
            position: sticky;
            top: 0;
            z-index: 1000;
            backdrop-filter: blur(10px);
        }

        .nav-container {
            max-width: 1200px;
            margin: 0 auto;
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 0 30px;
        }

        .nav-brand {
            color: var(--primary-red);
            font-size: 1.8rem;
            font-weight: 800;
            text-decoration: none;
            letter-spacing: -0.5px;
            transition: all 0.3s ease;
            text-shadow: 2px 2px 4px var(--shadow-light);
        }

        .nav-brand:hover {
            color: var(--dark-red);
            transform: scale(1.05);
        }

        .nav-toggle {
            display: none;
            background: var(--primary-red);
            border: none;
            color: white;
            font-size: 1.5rem;
            cursor: pointer;
            padding: 8px 12px;
            border-radius: 6px;
            transition: all 0.3s ease;
        }

        .nav-toggle:hover {
            background: var(--dark-red);
            transform: scale(1.1);
        }

        .nav-links {
            display: flex;
            list-style: none;
            margin: 0;
            padding: 0;
            gap: 2.5rem;
            align-items: center;
        }

        .nav-links li {
            margin: 0;
        }

        .nav-links a {
            color: var(--text-dark);
            text-decoration: none;
            font-weight: 600;
            font-size: 1.05rem;
            transition: all 0.3s ease;
            position: relative;
            padding: 8px 0;
        }

        .nav-links a::after {
            content: '';
            position: absolute;
            bottom: 0;
            left: 0;
            width: 0;
            height: 3px;
            background: var(--primary-red);
            transition: width 0.3s ease;
        }

        .nav-links a:hover {
            color: var(--primary-red);
        }

        .nav-links a:hover::after {
            width: 100%;
        }

        .user-info {
            color: var(--text-dark);
            font-weight: 600;
            background: var(--light-gray);
            padding: 8px 16px;
            border-radius: 20px;
            border: 2px solid var(--medium-gray);
        }

        .logout-button {
            background: var(--primary-red);
            color: white;
            border: none;
            padding: 10px 20px;
            border-radius: 25px;
            cursor: pointer;
            font-weight: 600;
            font-size: 1rem;
            box-shadow: 0 4px 8px var(--shadow-medium);
            transition: all 0.3s ease;
        }

        .logout-button:hover {
            background: var(--dark-red);
            transform: translateY(-2px);
            box-shadow: 0 6px 12px var(--shadow-medium);
        }

        .logout-button:active {
            transform: translateY(0);
        }

        @media (max-width: 768px) {
            .nav-toggle {
                display: block;
            }

            .nav-links {
                display: none;
                flex-direction: column;
                position: absolute;
                top: 70px;
                right: 20px;
                background: white;
                padding: 25px;
                width: 250px;
                border-radius: 12px;
                box-shadow: 0 8px 24px var(--shadow-dark);
                border: 2px solid var(--primary-red);
                gap: 1.5rem;
            }

            .nav-links.active {
                display: flex;
            }

            .nav-links a::after {
                display: none;
            }
        }
    "#);

    html! {
        <nav class={css}>
            <div class="nav-container">
                <Link<Route> to={Route::Home} classes={classes!("nav-brand")}>
                    { "Handyman Services" }
                </Link<Route>>

                <button class="nav-toggle" onclick={toggle_navbar} aria-label="Toggle navigation">
                    { "☰" }
                </button>

                <ul class={classes!("nav-links", active_class)}>
                    <li>
                        <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                    </li>
                    <li>
                        <Link<Route> to={Route::Booking}>{ "Book Service" }</Link<Route>>
                    </li>
                    <li>
                        <Link<Route> to={Route::Experience}>{ "Experience" }</Link<Route>>
                    </li>
                    <li>
                        <Link<Route> to={Route::Map}>{ "Service Map" }</Link<Route>>
                    </li>
                    <li>
                        <Link<Route> to={Route::Contact}>{ "Contact" }</Link<Route>>
                    </li>

                    if auth.is_authenticated {
                        if let Some(user) = &auth.user {
                            <li>
                                <span class="user-info">{ format!("Welcome, {}", user.username) }</span>
                            </li>
                            <li>
                                <button class="logout-button" onclick={on_logout}>{ "Logout" }</button>
                            </li>
                        }
                    } else {
                        <li>
                            <Link<Route> to={Route::Login}>{ "Login" }</Link<Route>>
                        </li>
                        <li>
                            <Link<Route> to={Route::Register}>{ "Register" }</Link<Route>>
                        </li>
                    }
                </ul>
            </div>
        </nav>
    }
}

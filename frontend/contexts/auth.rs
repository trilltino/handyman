//! # Authentication Context
//!
//! ## Purpose
//! Global authentication state using Yew's reducer pattern.
//! Tracks whether user is logged in and stores user information.
//!
//! ## State Management
//! - **AuthState**: Current auth state (user, is_authenticated)
//! - **AuthAction**: Actions to update state (Login, Logout)
//! - **AuthContext**: Handle for accessing/updating state
//!
//! ## How It Works
//! 1. AuthProvider wraps app and creates reducer
//! 2. Components use `use_context::<AuthContext>()` to access state
//! 3. Components dispatch actions: `auth.dispatch(AuthAction::Login(user))`
//! 4. Reducer updates state based on action
//! 5. All subscribed components re-render
//!
//! ## Relation to Entire Program
//! - **Provided By**: App component
//! - **Used By**: Nav (show login/logout), pages (auth checks)
//! - **Updated By**: Login/Register pages on successful auth

use std::rc::Rc;          // Reference counting for state
use yew::prelude::*;      // Yew framework
use crate::api::UserInfo; // User information struct

/// Authentication state
#[derive(Debug, Clone, PartialEq)]
pub struct AuthState {
    /// Current user (None if not logged in)
    pub user: Option<UserInfo>,
    /// Whether user is authenticated
    pub is_authenticated: bool,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            user: None,
            is_authenticated: false,
        }
    }
}

/// Actions that can be dispatched to update auth state
pub enum AuthAction {
    /// User logged in successfully
    Login(UserInfo),
    /// User logged out
    Logout,
}

impl Reducible for AuthState {
    type Action = AuthAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AuthAction::Login(user) => Self {
                user: Some(user),
                is_authenticated: true,
            }.into(),
            AuthAction::Logout => Self {
                user: None,
                is_authenticated: false,
            }.into(),
        }
    }
}

pub type AuthContext = UseReducerHandle<AuthState>;

#[derive(Properties, PartialEq)]
pub struct AuthProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AuthProvider)]
pub fn auth_provider(props: &AuthProviderProps) -> Html {
    let auth_state = use_reducer(AuthState::default);

    html! {
        <ContextProvider<AuthContext> context={auth_state}>
            {props.children.clone()}
        </ContextProvider<AuthContext>>
    }
}

//! Router module exposes Routers for the application.
//!
//! A Router receives http requests, perform authentication and pass the
//! operations to the Facade.
//!
//! Usually there are more than one according to application sections,
//! there is at least one router for SDK and another for Web Application.

mod sdk;
mod web_app;

// Re-export routers
pub use sdk::SDK_ROUTER;
pub use web_app::WEB_APP_ROUTER;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use sandbox_rust_web_app::{
    middleware::{add_cors_middleware, add_logging_middleware},
    router::{SDK_ROUTER, WEB_APP_ROUTER},
};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
    // build our application with a route
    let mut app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(handler))
        // SDK v0 user
        .nest("/sdk/v0", SDK_ROUTER.to_owned())
        // Web application router
        .nest("/", WEB_APP_ROUTER.to_owned());

    // add 404 for unknown path
    app = app.fallback(handler_404);
    // Add middlewares to our application
    // layers are accessed from bottom to up
    app = add_logging_middleware(app);
    app = add_cors_middleware(app);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

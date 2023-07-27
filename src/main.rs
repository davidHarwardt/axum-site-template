use std::net::SocketAddr;

use axum::{
    Router,
    routing::get,
    response::IntoResponse,
    http::{
        Response,
        StatusCode,
        header,
        HeaderValue,
        Request,
    },
    body::{
        self,
        Full,
        Body,
    },
};
use maud::{
    Markup,
    html,
    DOCTYPE,
};

async fn index() -> Markup {
    html! {
        (DOCTYPE);
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta name="X-UA-Compatible" content="id=edge";

                title { "axum-site" };
                link rel="stylesheet" href="/style.css";
            };
            body {
                h1."text-xl flex justify-center mt-8 gap-1" {
                    "edit";
                    code."bg-slate-200 rounded" { "src/main.rs" };
                    "to change the site";
                }
            };
        };
    }
}

async fn error_handler(req: Request<Body>) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, html! {
        h1 { "not found: "; (req.uri()); }
    })
}

async fn style_css() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, HeaderValue::from_str("text/css").unwrap())
        .body(body::boxed(Full::from(include_str!("../dist/style.css"))))
    .unwrap()
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let app = Router::new()
        .route("/", get(index))
        .route("/style.css", get(style_css))
        .fallback(error_handler);

    #[cfg(debug_assertions)]
    let app = app.layer(tower_livereload::LiveReloadLayer::new());

    let host = SocketAddr::from(([0, 0, 0, 0], 3000));

    tracing::info!("starting server (http://{host})");
    axum::Server::bind(&host)
        .serve(app.into_make_service())
        .await
    .expect("server error");
}


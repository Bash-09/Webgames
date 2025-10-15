use std::path::{Path, PathBuf};

use axum::{
    Router,
    extract::State,
    http::{StatusCode, header},
    response::{IntoResponse, Redirect, Response},
    routing::get,
};
use include_dir::{Dir, include_dir};

static BUNDLED_FILES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/../frontend/dist");

pub fn router(ui_source: UISource) -> Router {
    Router::new()
        .route("/", get(ui_redirect))
        .route("/{*wildcard}", get(get_ui))
        .with_state(ui_source)
}

async fn ui_redirect() -> impl IntoResponse {
    Redirect::permanent("/index.html")
}

async fn get_ui(
    State(state): State<UISource>,
    axum::extract::Path(path): axum::extract::Path<String>,
) -> Response {
    state.get_ui(&path).await
}

/// Where to get the web UI files to serve
#[derive(Clone, Debug, Default)]
pub enum UISource {
    Bundled(&'static Dir<'static>),
    /// Load from disk
    Dynamic(PathBuf),
    #[default]
    None,
}

impl UISource {
    pub fn bundled() -> Self {
        UISource::Bundled(&BUNDLED_FILES)
    }

    pub async fn get_ui(&self, path: &str) -> Response {
        match self {
            Self::Bundled(dir) => Self::get_bundled_ui(dir, path).into_response(),
            Self::Dynamic(dir) => Self::get_dynamic_ui(dir, path).await.into_response(),
            Self::None => {
                (
                    StatusCode::NOT_FOUND,
                    ([(header::CONTENT_TYPE, "text/html")]),
                    "<body><h1>There is no UI bundled with this version of the application.</h1></body>",
                ).into_response()
            }
        }
    }

    fn get_bundled_ui(dir: &'static Dir<'static>, path: &str) -> impl IntoResponse {
        dir.get_file(path).map_or_else(
            || {
                (
                    StatusCode::NOT_FOUND,
                    ([(header::CONTENT_TYPE, "text/html")]),
                    "<body><h1>404 Not Found</h1></body>",
                )
                    .into_response()
            },
            |file| {
                // Serve included file
                let content_type = guess_content_type(file.path());
                let headers = [
                    (header::CONTENT_TYPE, content_type),
                    (header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
                ];
                (StatusCode::OK, headers, file.contents()).into_response()
            },
        )
    }

    async fn get_dynamic_ui(dir: &Path, path: &str) -> impl IntoResponse {
        let file_path = dir.join(path);

        if file_path.is_file() {
            let contents = tokio::fs::read(&file_path).await;

            match contents {
                Ok(contents) => {
                    let content_type = guess_content_type(&file_path);
                    let headers = [
                        (header::CONTENT_TYPE, content_type),
                        (header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
                    ];
                    (StatusCode::OK, headers, contents).into_response()
                }
                Err(_) => {
                    // tracing::error!("Failed to read file {:?}: {e}", &file_path);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ([(header::CONTENT_TYPE, "text/html")]),
                        "<body><h1>500 Internal Server Error</h1><p>Failed to read file in override web-ui directory</p></body>",
                    )
                        .into_response()
                }
            }
        } else {
            (
                StatusCode::NOT_FOUND,
                ([(header::CONTENT_TYPE, "text/html")]),
                "<body><h1>404 Not Found</h1></body>",
            )
                .into_response()
        }
    }
}

/// Attempts to guess the http MIME type of a given file extension.
/// Defaults to "application/octet-stream" if it is not recognised.
fn guess_content_type(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|osstr| osstr.to_str())
        .unwrap_or("bin")
    {
        "htm" | "html" => "text/html",
        "jpg" | "jpeg" => "image/jpeg",
        "js" => "text/javascript",
        "json" => "application/json",
        "png" => "image/png",
        "weba" => "audio/weba",
        "webm" => "video/webm",
        "webp" => "image/webp",
        "txt" => "text/plain",
        "mp3" => "audio/mp3",
        "mp4" => "video/mp4",
        "ttf" => "font/ttf",
        "otf" => "font/otf",
        "css" => "text/css",
        _ => "application/octet-stream",
    }
}

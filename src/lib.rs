use http_tokio::{Request, Response};

/// Fails only for io file errors (file not found)
pub async fn serve_static(req: &Request, base_path: &str) -> Result<Response, std::io::Error> {
    let mut req_path = req.path.trim_start_matches("/").trim_end_matches("/");
    if req_path.is_empty() { req_path = "index.html"; }
    let base_path = base_path.trim_end_matches("/");
    let mut static_file_path = format!("{base_path}/{req_path}");
    if !static_file_path.split("/").last().unwrap().contains(".") {
        static_file_path += ".html";
    }
    Response::build().file(static_file_path).await
}
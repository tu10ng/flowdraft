use std::fs;
use tiny_http::{Response, Server};

fn main() {
    let root = std::env::current_exe()
        .expect("Failed to get executable path")
        .parent()
        .expect("Failed to get executable directory")
        .to_path_buf();

    // Bind to localhost with auto-assigned port
    let server = Server::http("127.0.0.1:0").expect("Failed to start server");
    let addr = server.server_addr();
    let url = format!("http://{}", addr);

    println!("Starting Flowdraft web server at {}", url);
    println!("Press Ctrl+C to stop");

    // Open browser
    if let Err(e) = open::that(&url) {
        eprintln!("Failed to open browser: {}", e);
        println!("Please open {} manually", url);
    }

    // Serve files
    for request in server.incoming_requests() {
        let mut path = request.url().trim_start_matches('/').to_string();

        // Default to index.html
        if path.is_empty() || path.ends_with('/') {
            path.push_str("index.html");
        }

        let file_path = root.join(&path);

        match fs::read(&file_path) {
            Ok(content) => {
                let content_type = match file_path.extension().and_then(|s| s.to_str()) {
                    Some("html") => "text/html; charset=utf-8",
                    Some("js") => "application/javascript; charset=utf-8",
                    Some("wasm") => "application/wasm",
                    Some("css") => "text/css; charset=utf-8",
                    Some("svg") => "image/svg+xml",
                    Some("png") => "image/png",
                    Some("jpg") | Some("jpeg") => "image/jpeg",
                    Some("json") => "application/json",
                    _ => "application/octet-stream",
                };

                let response = Response::from_data(content)
                    .with_header(
                        tiny_http::Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes())
                            .unwrap()
                    );

                let _ = request.respond(response);
            }
            Err(_) => {
                let response = Response::from_string("404 Not Found")
                    .with_status_code(404);
                let _ = request.respond(response);
            }
        }
    }
}

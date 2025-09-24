 * It handles HTTP requests and returns responses based on the client's device characteristics.
 *
 * The service checks the 'User-Agent' header to determine the device type and
 * returns a layout appropriate for desktop, tablet, or mobile devices.
 */

use actix_web::{
    get,
    web,
    HttpResponse,
    Responder,
    http::header,
};

/// Determines the layout based on the user's device characteristics.
///
/// # Arguments
///
/// * `user_agent` - A reference to the User-Agent header of the request.
///
/// # Returns
///
/// A string indicating the layout type (desktop, tablet, or mobile).
fn determine_layout(user_agent: &str) -> &'static str {
    let layout_type = if user_agent.contains("Mobile") {
        "mobile"
    } else if user_agent.contains("Tablet") {
        "tablet"
    } else {
        "desktop"
    };
    // More complex device detection logic could be added here.
    layout_type
}

#[get("/layout")]
/// Handles HTTP GET requests to determine the layout.
///
/// # Arguments
///
/// * `cfg` - The configuration object containing the request data.
///
/// # Returns
///
/// An HTTP response with the layout type.
async fn layout(cfg: web::Data<ActixConfig>) -> impl Responder {
    let user_agent = cfg.request().headers().get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    // Determine the layout based on the User-Agent header.
    let layout = determine_layout(user_agent);

    // Return a simple HTTP response with the layout type.
    HttpResponse::Ok().json(format!("The layout type is: {}", layout))
}

/// Represents the configuration for the Actix application.
struct ActixConfig;

/// Starts the Actix web service.
///
/// This function sets up the Actix application and registers the layout endpoint.
fn start_service() -> std::io::Result<()> {
    // Set up the Actix app with the layout endpoint.
    let app = actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .data(ActixConfig)
            .service(layout)
    });

    // Run the Actix web service on port 8080.
    app.bind("127.0.0.1:8080")?.run()?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    start_service()
}

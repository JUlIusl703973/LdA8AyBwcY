use actix_web::{
    get,
    web,
    App,
    HttpServer,
    HttpResponse,
    Responder,
};
use std::collections::HashMap;
use htmlescape::encode_html;

// Define a custom error for XSS protection errors.
#[derive(Debug)]
enum XssError {
    EscapeError,
}

// Implement Responder trait for XssError to return a proper HTTP response.
impl Responder for XssError {
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        match self {
            XssError::EscapeError => HttpResponse::InternalServerError()
                .content_type("text/plain")
                .body("An error occurred during HTML escaping."),
        }
    }
}

#[get("/")]
// Define a simple route handler to demonstrate XSS protection.
async fn index() -> Result<&'static str, XssError> {
    let user_input = "<script>alert('XSS')</script>";
    // Escape the user input to prevent XSS.
    let escaped_input = encode_html(user_input)
        .map_err(|_| XssError::EscapeError)?;

    // Return the escaped input to the client.
    Ok("<p>Escaped user input: ".to_string() + &escaped_input + "</p>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the HTTP server with the defined routes.
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

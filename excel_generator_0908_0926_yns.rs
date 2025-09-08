// excel_generator.rs
define_excel_generator() -> impl ActixService<Request = actix_web::HttpRequest, Response = actix_web::HttpResponse, Error = actix_web::Error> + 'static {\
    actix_web::web::resource("/generate").route(
        actix_web::http::Method::POST,
        actix_web::web::to(generate_excel)
    )
}

/// Generates an Excel file based on the provided data.
async fn generate_excel(req: actix_web::HttpRequest) -> Result<actix_web::HttpResponse, actix_web::Error> {\
    let body: String = req.body().await?;
    let data: serde_json::Value = serde_json::from_str(&body).map_err(|e| {\
        actix_web::HttpResponse::BadRequest().json(e.to_string())
    })?;

    let sheet_name = data.get("sheet_name").and_then(|n| n.as_str()).unwrap_or("Sheet1");
    let rows = data.get("rows").and_then(|r| r.as_array()).unwrap_or(&vec![]);

    // Generate the Excel file content
    let mut excel_content = String::new();
    excel_content.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\
");
    excel_content.push_str("<worksheet xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\">\
");
    excel_content.push_str("<sheetData>\
");

    for row in rows.iter() {
        let mut row_str = String::new();
        row_str.push_str("<row>\
");
        for cell in row.iter() {
            if let Some(value) = cell.as_str() {
                row_str.push_str(&format!("<c t=\"s\"><v>{}</v></c>\
", value));
            }
        }
        row_str.push_str("</row>\
");
        excel_content.push_str(&row_str);
    }

    excel_content.push_str("</sheetData>\
");
    excel_content.push_str("</worksheet>\
");

    // Set the response headers and content type
    let mut response = actix_web::HttpResponse::Ok();
    response.insert_header(actix_web::http::header::CONTENT_TYPE, "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet");
    response.insert_header(actix_web::http::header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}.xlsx\"", sheet_name));

    response.body(actix_web::body::boxed(excel_content))
}

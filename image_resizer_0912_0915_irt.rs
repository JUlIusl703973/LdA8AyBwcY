use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use image::{open, imageops::resize, ImageOutputFormat};
use std::path::Path;
use anyhow::Result;

// 定义一个结构体来处理图片尺寸调整的请求
struct ImageResizer;

impl ImageResizer {
    // 处理图片尺寸调整的方法
    async fn handle_request(path: String, new_dimensions: (u32, u32)) -> impl Responder {
        let path = Path::new(&path);
        match open(path) {
            Ok(mut img) => {
                // 调整图片尺寸
                let resized_img = resize(&mut img, new_dimensions.0, new_dimensions.1, image::imageops::FilterType::Nearest);
                // 将调整尺寸后的图片输出为PNG格式
                let format = if path.extension().and_then(std::ffi::OsStr::to_str) == Some("jpg") {
                    ImageOutputFormat::Jpeg(85)
                } else {
                    ImageOutputFormat::Png
                };
                let mut buffer = Vec::new();
                if resized_img.write_to(&mut buffer, format).is_ok() {
                    HttpResponse::Ok().content_type("image/png").body(buffer)
                } else {
                    HttpResponse::InternalServerError().body("Failed to write image")
                }
            }
            Err(_) => HttpResponse::BadRequest().body("Invalid image path"),
        }
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/resize", web::post().to(ImageResizer::handle_request))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use image::io::Reader as ImageReader;
    use bytes::buf::BufExt;

    #[actix_web::test]
    async fn test_image_resizer() {
        let app = test::init_service(App::new()
            .service(web::resource("/resize").route(web::post().to(ImageResizer::handle_request))).await;

        // 测试用的图片路径
        let image_path = "test_image.jpg";
        // 新的图片尺寸
        let new_dimensions = (100, 100);

        let req = test::TestRequest::with_uri(&format!("/resize?path={}&width={}&height={}", image_path, new_dimensions.0, new_dimensions.1))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // 读取响应体中的图片数据
        let mut body = resp.response().body().as_ref();
        let image = ImageReader::open(&mut body.reader()).expect("Failed to read image");
        let decoded_image = image.decode().expect("Failed to decode image");

        // 验证图片尺寸是否正确
        assert_eq!(decoded_image.width(), new_dimensions.0);
        assert_eq!(decoded_image.height(), new_dimensions.1);
    }
}
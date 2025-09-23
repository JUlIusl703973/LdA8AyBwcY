use actix_web::{get, HttpResponse, Responder};

/// 响应式布局控制器
/// 这个控制器负责处理所有与响应式布局相关的请求
struct ResponsiveLayoutController;

/// 定义 GET 请求的处理函数
/// 这个函数将返回一个简单的响应，模拟响应式布局的实现
#[get("/layout")]
async fn layout() -> impl Responder {
    let content = r#"
    <html>
        <head>
            <title>Responsive Layout</title>
            <style>
                body {
                    background-color: #f0f0f0;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    height: 100vh;
                    margin: 0;
                }
                .responsive-container {
                    padding: 20px;
                    background-color: white;
                    border-radius: 10px;
                    box-shadow: 0 4px 8px rgba(0,0,0,0.1);
                }
                @media (max-width: 600px) {
                    .responsive-container {
                        width: 100%;
                    }
                }
            </style>
        </head>
        <body>
            <div class="responsive-container">
                <h1>Responsive Layout Example</h1>
                <p>This is a simple responsive layout example using CSS media queries.</p>
            </div>
        </body>
    </html>
    "#;
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(content)
}

/// main 函数，程序的入口点
/// 设置服务器并启动监听
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启用日志记录
    env_logger::init();

    // 启动服务器
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(layout)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

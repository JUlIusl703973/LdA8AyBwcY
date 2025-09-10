use actix_web::{web, App, HttpResponse, HttpServer, Responder};

/// 排序算法，使用冒泡排序作为示例。
/// 可以根据需要替换为其他排序算法，比如快速排序、归并排序等。
fn bubble_sort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len-i-1 {
# 优化算法效率
            if arr[j] > arr[j+1] {
# 添加错误处理
                arr.swap(j, j+1);
            }
        }
    }
# NOTE: 重要实现细节
}

/// 排序服务的响应结构体。
/// 用于传递排序结果。
#[derive(serde::Serialize)]
struct SortResponse {
    sorted_array: Vec<i32>,
}

/// 排序服务的处理函数。
/// 接受一个包含整数的Vec作为输入，并返回排序后的数组。
async fn sort_service(array: web::Json<Vec<i32>>) -> impl Responder {
# TODO: 优化性能
    let mut array = array.into_inner();
    bubble_sort(&mut array);
    Ok(HttpResponse::Ok().json(SortResponse { sorted_array: array }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动HTTP服务器
    HttpServer::new(|| {
        App::new()
            .route("/sort", web::post().to(sort_service))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

use actix_web::{web, App, HttpServer, Responder, HttpResponse, Result};

// 定义一个排序服务结构体
struct SortService;

impl SortService {
    // 实现一个冒泡排序算法
    fn bubble_sort<T: Ord + Clone>(input: Vec<T>) -> Vec<T> {
        let mut input = input;
        let len = input.len();
        for i in 0..len {
            for j in 0..len-i-1 {
                if input[j] > input[j+1] {
                    input.swap(j, j+1);
                }
            }
        }
        input
    }

    // 实现一个快速排序算法
    fn quick_sort<T: Ord + Clone + Copy>(input: Vec<T>) -> Vec<T> {
        if input.len() <= 1 {
            return input;
        }
        let pivot = input[input.len() / 2];
        let (mut less, mut equal, mut greater) = (Vec::new(), Vec::new(), Vec::new());
        for x in input {
            if x < pivot {
                less.push(x);
            } else if x > pivot {
                greater.push(x);
            } else {
                equal.push(x);
            }
        }
        SortService::quick_sort(less) + equal + SortService::quick_sort(greater)
    }
}

// 定义一个处理排序请求的函数
async fn sort_numbers(sort_service: web::Data<SortService>, input: web::Json<Vec<i32>>) -> Result<impl Responder> {
    let sorted = SortService::bubble_sort(input.into_inner());
    Ok(HttpResponse::Ok().json({"sorted": sorted})?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(SortService))
            .route("/sort", web::post().to(sort_numbers))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 以下是文档注释
// 该程序使用 ACTIX 框架创建一个 HTTP 服务，提供排序算法的实现。
// 支持冒泡排序和快速排序两种算法。
// 可以通过 POST 请求发送一个整数数组到 /sort 路径来调用排序服务。
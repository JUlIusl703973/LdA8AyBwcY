use actix_web::{get, HttpResponse, Responder, web};

/// 排序算法服务结构体
struct SortingService;

/// 实现排序算法的方法
impl SortingService {
    /// 冒泡排序算法
    #[allow(dead_code)]
    fn bubble_sort<T: Ord + Copy + std::fmt::Debug>(items: &mut [T]) -> Vec<T> {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 1..items.len() {
                if items[i - 1] > items[i] {
                    items.swap(i - 1, i);
                    swapped = true;
                }
            }
        }
        items.to_vec()
    }

    /// 快速排序算法
    #[allow(dead_code)]
    fn quick_sort<T: Ord + PartialOrd + Copy + std::fmt::Debug>(items: &mut [T]) -> Vec<T> {
        if items.len() <= 1 {
            items.to_vec()
        } else {
            let pivot_index = items.len() / 2;
            let pivot = items[pivot_index];
            let mut (mut less, mut greater): (Vec<T>, Vec<T>) = (Vec::new(), Vec::new());
            for &item in items.iter() {
                if item < pivot {
                    less.push(item);
                } else if item > pivot {
                    greater.push(item);
                }
            }
            let mut sorted: Vec<T> = Self::quick_sort(&mut less)
                .into_iter()
                .chain(Self::quick_sort(&mut greater).into_iter())
                .collect();
            sorted.push(pivot);
            sorted
        }
    }
}

/// 排序算法处理方法
#[get("/sort/{algorithm}")]
async fn sort_algorithm(sort_algorithm: web::Path<String>) -> impl Responder {
    let algorithm = sort_algorithm.into_inner();
    let vec_to_sort = vec![5, 3, 4, 2, 1];
    let sorted_vec = match algorithm.as_str() {
        "bubble" => SortingService::bubble_sort(&mut vec_to_sort.clone()),
        "quick" => SortingService::quick_sort(&mut vec_to_sort.clone()),
        _ => return HttpResponse::BadRequest().json("Invalid sorting algorithm"),
    };
    HttpResponse::Ok().json(sorted_vec)
}

/// 程序入口点
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动HTTP服务器
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(sort_algorithm)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

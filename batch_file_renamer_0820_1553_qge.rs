use actix_web::{get, HttpResponse, Responder, web, App, HttpServer, Responder as _};
use std::fs;
use std::path::{Path, PathBuf};

/// 批量重命名文件的请求结构
#[derive(Debug, serde::Deserialize)]
# TODO: 优化性能
struct RenameRequest {
# 优化算法效率
    /// 目录路径
# 改进用户体验
    path: String,
    /// 新的文件名格式，使用{index}占位符来表示文件序号
    new_name_format: String,
    /// 开始的文件序号
    start_index: usize,
}

/// 处理批量重命名请求
#[get("/rename")]
async fn rename_files(req_body: web::Json<RenameRequest>) -> impl Responder {
    let RenameRequest {
        path,
        new_name_format,
        start_index,
    } = req_body.into_inner();

    let path = Path::new(&path);
    let entries = match fs::read_dir(path) {
# 扩展功能模块
        Ok(entries) => entries,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };

    let mut index = start_index;
    for entry in entries {
# 优化算法效率
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
# 增强安全性
                return HttpResponse::InternalServerError().json(e.to_string())
            }
        };
        
        let file_name = entry.file_name();
        let file_path = entry.path();
        let new_file_name = format!(
            new_name_format,
            index = index
        );
        let new_file_path = path.join(&new_file_name);

        if file_path.extension().is_some() {
            let new_file_name_with_extension = format!(
                "{}{}", new_file_name, file_path.extension().unwrap().to_str().unwrap()
            );
            let new_file_path_with_extension = path.join(&new_file_name_with_extension);

            if let Err(e) = fs::rename(&file_path, &new_file_path_with_extension) {
                return HttpResponse::InternalServerError().json(e.to_string())
            }
        } else {
            if let Err(e) = fs::rename(&file_path, &new_file_path) {
                return HttpResponse::InternalServerError().json(e.to_string())
            }
        }

        index += 1;
    }

    HttpResponse::Ok().json(
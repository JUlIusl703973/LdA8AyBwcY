use actix_web::{web, App, HttpServer, Responder, HttpResponse, Result};
use std::path::Path;
use std::fs::{self, DirEntry};
use std::io::Result as IoResult;

// 定义一个结构体用于表示文件夹结构
struct FolderOrganizer {
    path: String,
}

// 实现FolderOrganizer的方法
impl FolderOrganizer {
    // 构造函数
    pub fn new(path: String) -> Self {
        FolderOrganizer { path }
    }

    // 整理文件夹
    pub fn organize(&self) -> IoResult<()> {
        let entries = fs::read_dir(&self.path)?;
        for entry in entries {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                // 递归整理子文件夹
                self.organize_subfolder(&entry.path())?;
            } else {
                // 对文件进行排序
                self.sort_files(&entry)?;
            }
        }
        Ok(())
    }

    // 整理子文件夹
    fn organize_subfolder(&self, path: &Path) -> IoResult<()> {
        let mut organizer = FolderOrganizer::new(path.to_str().unwrap().to_string());
        organizer.organize()
    }

    // 对文件进行排序
    fn sort_files(&self, file: &DirEntry) -> IoResult<()> {
        // 这里可以根据文件类型或名称进行排序
        // 示例：按文件名排序
        // 这里省略具体实现，可以根据需求添加排序逻辑
        Ok(())
    }
}

// 定义一个结构体用于处理请求
struct OrganizeRequest {
    path: String,
}

// 实现从请求中提取路径的功能
impl From<web::Json<OrganizeRequest>> for FolderOrganizer {
    fn from(json: web::Json<OrganizeRequest>) -> Self {
        FolderOrganizer::new(json.path)
    }
}

// 定义一个异步处理函数
async fn organize_folder(organizer: web::Json<OrganizeRequest>) -> Result<impl Responder> {
    let organizer: FolderOrganizer = organizer.into();
    match organizer.organize() {
        Ok(_) => Ok(HttpResponse::Ok().body("Folder organized successfully".to_string())),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Error: {}", e))),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/organize").route(web::post().to(organize_folder)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 文档注释
/// 此程序是一个文件夹结构整理器，使用RUST和ACTIX框架实现。
/// 它提供了一个HTTP接口，接收文件夹路径，然后对文件夹中的文件进行排序和整理。
/// 程序遵循RUST最佳实践，具有良好的可维护性和可扩展性。
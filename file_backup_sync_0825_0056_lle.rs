use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

// 定义一个结构体来管理备份和同步
struct FileBackupSync {
    source_path: PathBuf,
    backup_path: PathBuf,
}

impl FileBackupSync {
    // 创建一个新的FileBackupSync实例
    pub fn new(source_path: PathBuf, backup_path: PathBuf) -> Self {
        FileBackupSync {
            source_path,
            backup_path,
        }
    }

    // 同步文件到备份目录
    pub fn sync_to_backup(&self) -> io::Result<()> {
        // 检查源文件是否存在
        if !self.source_path.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Source file not found"));
        }

        // 创建备份目录，如果不存在
        fs::create_dir_all(&self.backup_path)?;

        // 读取源文件
        let mut source_file = File::open(&self.source_path)?;
        let mut contents = Vec::new();
        source_file.read_to_end(&mut contents)?;

        // 写入备份文件
        let backup_file = self.backup_path.join(self.source_path.file_name().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "File name not found")
        })?);

        let mut backup_file = File::create(&backup_file)?;
        backup_file.write_all(&contents)?;
        backup_file.flush()?;

        Ok(())
    }

    // 获取最近一次备份的时间戳
    pub fn last_backup_timestamp(&self) -> io::Result<SystemTime> {
        self.backup_path
            .read_dir()?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.is_file())
            .filter_map(|path| path.metadata().ok())
            .filter(|metadata| {
                metadata.permissions().readonly() ||
                metadata.len() > 0
            })
            .map(|metadata| metadata.modified().ok())
            .max()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No backup files found"))
    }
}

// 实现Actix Web服务
async fn backup_and_sync() -> impl Responder {
    let file_sync = FileBackupSync::new(
        PathBuf::from("./source.txt"), // 源文件路径
        PathBuf::from("./backup"),    // 备份目录路径
    );

    match file_sync.sync_to_backup() {
        Ok(_) => HttpResponse::Ok().json{"success": true, "message": "File synced successfully"},
        Err(e) => HttpResponse::InternalServerError().json{"error": e.to_string()},
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/backup", web::get().to(backup_and_sync))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
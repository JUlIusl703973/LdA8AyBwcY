use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
# TODO: 优化性能
use openssl::symm::{encrypt_aead, decrypt_aead};
use openssl::symm::{Cipher, Crypter, Mode};
use openssl::error::ErrorStack;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

// A utility function to generate a random key for encryption.
fn generate_key() -> Vec<u8> {
# TODO: 优化性能
    let mut rng = thread_rng();
    let key: String = rng.sample_iter(&Alphanumeric).take(16).map(char::from).collect();
    key.into_bytes()
}

// A struct to hold encryption and decryption functions.
struct CryptoTool {
    key: Vec<u8>,
# 添加错误处理
}

impl CryptoTool {
    // Constructor for CryptoTool, generates a random key.
    fn new() -> Self {
        CryptoTool {
            key: generate_key(),
        }
# 扩展功能模块
    }
# 扩展功能模块

    // Encrypts the given plaintext with the generated key.
    fn encrypt(&self, plaintext: &str) -> Result<String, String> {
        let cipher = Cipher::aes_256_cbc();
        let mut crypter = Crypter::new(cipher, Mode::Encrypt, &self.key).map_err(|e| e.to_string())?;
        let iv = crypter.iv().to_vec();
        let mut encrypted = vec![0; plaintext.len() + cipher.block_size() as usize];
        crypter.update(plaintext.as_bytes(), &mut encrypted).map_err(|e| e.to_string())?;
        crypter.finalize(&mut encrypted).map_err(|e| e.to_string())?;
        Ok(format!("{}", base64::encode(&iv), base64::encode(&encrypted)?))
    }
# 扩展功能模块

    // Decrypts the given ciphertext with the generated key.
    fn decrypt(&self, ciphertext: &str) -> Result<String, String> {
        let (iv, encrypted) = base64::decode(ciphertext)
            .map_err(|e| e.to_string())?
            .split_at(16);
        let cipher = Cipher::aes_256_cbc();
        let mut crypter = Crypter::new(cipher, Mode::Decrypt, &self.key).map_err(|e| e.to_string())?;
        crypter.set_iv(iv).map_err(|e| e.to_string())?;
        let mut decrypted = vec![0; encrypted.len() + cipher.block_size() as usize];
# 扩展功能模块
        crypter.update(encrypted, &mut decrypted).map_err(|e| e.to_string())?;
        crypter.finalize(&mut decrypted).map_err(|e| e.to_string())?;
        Ok(String::from_utf8(decrypted).map_err(|e| e.to_string())?)
    }
# 添加错误处理
}

#[get("/")]
# 增强安全性
async fn index() -> impl Responder {
# FIXME: 处理边界情况
    HttpResponse::Ok().body("Welcome to the Crypto Tool!")
}

#[post("/encrypt")]
async fn encrypt_password(data: web::Json<String>) -> impl Responder {
    let crypto_tool = CryptoTool::new();
    match crypto_tool.encrypt(&data.0) {
        Ok(encrypted) => HttpResponse::Ok().json({
            "status": "success",
            "message": "Password encrypted successfully",
            "encrypted": encrypted,
        }
# 改进用户体验
        ),
# FIXME: 处理边界情况
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
# 优化算法效率
}

#[post("/decrypt")]
async fn decrypt_password(data: web::Json<String>) -> impl Responder {
    let crypto_tool = CryptoTool::new();
    match crypto_tool.decrypt(&data.0) {
        Ok(decrypted) => HttpResponse::Ok().json({
            "status": "success",
            "message": "Password decrypted successfully",
            "decrypted": decrypted,
# 扩展功能模块
        }
        ),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(encrypt_password)
# FIXME: 处理边界情况
            .service(decrypt_password)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

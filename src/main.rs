use actix_web::{middleware, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
// use std::net::SocketAddr;
use log::{debug, info};

const API_KEY: &str = "my_secret_api_key";

async fn api_key_auth(req: HttpRequest) -> impl Responder {
    let query_params = req.query_string();
    let headers = req.headers();

    let api_key = headers
        .get("X-API-KEY")   // 从请求头获取 API Key
        .and_then(|v| v.to_str().ok())
        .or_else(|| {
            // 或者 从 URL 查询参数获取 API Key
            query_params.split('&')
                .find(|param| param.starts_with("api_key="))
                .map(|param| param.trim_start_matches("api_key="))
        });

    match api_key {
        Some(key) if key == API_KEY => HttpResponse::Ok().body("API Key Authenticated!"),
        _ => HttpResponse::Unauthorized().body("Invalid API Key"),
    }
}

//-------------------------------------

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix-web!")
}

//-------------------------------------

#[derive(Deserialize)]
struct Info {
    name: String,
    age: u8,
}

#[post("/submit")]
async fn submit(info: web::Json<Info>) -> impl Responder {
    HttpResponse::Ok().json(format!("Received: {} is {} years old.", info.name, info.age))
}

//-------------------------------------

async fn greet(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}

//-------------------------------------

#[derive(Deserialize)]
struct FormData {
    username: String,
    // password: String,
}

#[post("/login")]
async fn login(form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().body(format!("Welcome, {}!", form.username))
}

//-------------------------------------

// 定义响应结构
#[derive(Serialize)]
struct ClientInfo {
    ip: String,
    user_agent: String,
    headers: Vec<(String, String)>,
}

// 处理请求的函数
async fn client_info(req: HttpRequest) -> impl Responder {
    //出于演示目标, 才收集以下的信息, 实际不需要,直接构造返回值就行

    // 获取IP地址
    let conn_info = req.connection_info();
    let ip = conn_info.realip_remote_addr().unwrap_or("unknown").to_string();

    // 获取User-Agent
    let user_agent = req
        .headers()
        .get("User-Agent")
        .and_then(|ua| ua.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    // 获取所有HTTP头
    let headers: Vec<(String, String)> = req
        .headers()
        .iter()
        .map(|(key, value)| {
            (
                key.to_string(),
                value.to_str().unwrap_or("invalid UTF-8").to_string(),
            )
        })
        .collect();

    // 创建响应
    let client_info = ClientInfo {
        ip,
        user_agent,
        headers,
    };

    HttpResponse::Ok().json(client_info)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 获取系统中可用的CPU核心数
    let num_cpus = num_cpus::get();
    env_logger::init(); // 初始化日志
    debug!("starting up");

    // 创建HTTP服务器，并配置工作线程数与路由
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default()) // 启用日志

            .route("/", web::get().to(hello))
            .service(submit)        //post 读取json请求
            .service(login)       //post 读取表单请求
            .route("/hello/{name}", web::get().to(greet))   //路径参数
            .route("/client-info", web::get().to(client_info))  //返回json结构

            .route("/protected", web::get().to(api_key_auth))
    })
    .workers(num_cpus)  // 设置工作线程数
    .bind("0.0.0.0:8080")?  // 绑定到指定地址和端口
    .run()
    .await
}


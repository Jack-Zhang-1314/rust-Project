use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

//配置路由
pub fn general_routes(cfg: &mut web::ServiceConfig) {
  cfg.route("/health", web::get().to(health_check_handler));
}

//配置handler
pub async fn health_check_handler() -> impl Responder {
  HttpResponse::Ok().json("Actix-web is running")
}

//实例化HTTP server 并运行
#[actix_rt::main]
async fn main() -> io::Result<()> {
  //构建app,配置route
  let app = move || App::new().configure(general_routes);

  //启动HTTP server
  HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}

//cargo run -p webservice --bin service1
//或者进入具体项目: cargo run --bin service1

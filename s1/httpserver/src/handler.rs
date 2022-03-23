use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;

/* The above code is defining a trait called Handler. This trait is used to define the behavior of a
handler. */
pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;
    //加载静态的文件
    fn load_page(file_name: &str) -> Option<String> {
        //crate根目录
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        //unwrap_or如果没有PUBLIC_PATH返回默认路径
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);
        //打印
        println!("{}", full_path);
        let contains = fs::read_to_string(full_path);
        contains.ok()
    }
}

pub struct StaticPageHandler;
pub struct PageNotFoundHandler;
pub struct WebSocketHandler;

#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String,
}

impl Handler for PageNotFoundHandler {
    fn handle(_req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_page("404.html"))
    }
}

impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();
        //打印
        println!("hello{:?}", route);
        match route[1] {
            "" => HttpResponse::new("200", None, Self::load_page("index.html")),
            "health" => HttpResponse::new("200", None, Self::load_page("health.html")),
            path => match Self::load_page(path) {
                Some(contens) => {
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    if path.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    } else if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else {
                        map.insert("Content-Type", "text/html");
                    }
                    HttpResponse::new("200", Some(map), Some(contens))
                }
                None => HttpResponse::new("404", None, Self::load_page("404.html")),
            },
        }
    }
}

impl WebSocketHandler {
    fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "orders.json");
        let json_contens = fs::read_to_string(full_path);
        let orders: Vec<OrderStatus> =
            serde_json::from_str(json_contens.unwrap().as_str()).unwrap();
        orders
    }
}

impl Handler for WebSocketHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();
        match route[2] {
            "shipping" if route.len() > 2 && route[3] == "orders" => {
                let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
                let mut headers: HashMap<&str, &str> = HashMap::new();
                headers.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(headers), body)
            }
            _ => HttpResponse::new("404", None, Self::load_page("404.html")),
        }
    }
}

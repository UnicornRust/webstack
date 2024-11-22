use std::{collections::HashMap, env, fs};

use http::{http_request::{self, HttpRequest}, http_response::HttpResponse};
use serde::{Deserialize, Serialize};


pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;
    fn load_file(file_name: &str) -> Option<String> {
        //获取当前项目的根目录, 由 cargo 内部的环境变量提供
        let default_path = format!("{}/public", env!("CARGO_MINIFEST_DIR"));
        // 获取系统环境变量 PUBLIC_PATH
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);
        let contents = fs::read_to_string(full_path);
        contents.ok()
    }

}

pub struct StaticPageHandler;
pub struct PageNotFoundHandler;
pub struct WebServiceHandler;

#[derive(Serialize, Deserialize)]
pub struct OrderState {
    order_id: i32,
    order_date: String,
    order_status: String,
}


impl Handler for PageNotFoundHandler {
    fn handle(_req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}


impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http_request::Resource::Path(path) = &req.resource;
        let route: Vec<&str> = path.split("/").collect();
        match route[1] {
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),
            "health" => HttpResponse::new("200", None, Self::load_file("health.html")),
            path => match Self::load_file(path) { 
                Some(content) => {
                    let mut map = HashMap::new();
                    if path.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    }else if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    }else {
                        map.insert("Content-Type", "text/html");
                    }
                    HttpResponse::new("200", Some(map), Some(content))
                }
                None => {
                    HttpResponse::new("404", None, Self::load_file("404.html"))
                }
                
            }
        }
    }
}

impl WebServiceHandler {
    fn load_json() -> Vec<OrderState> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "order.json");
        let json_content = fs::read_to_string(full_path);
        let orders: Vec<OrderState> = serde_json::from_str(json_content.unwrap().as_str()).unwrap();
        orders
    }
}

impl Handler for WebServiceHandler {

}

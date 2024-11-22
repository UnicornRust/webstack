


use std::io::Write;

use http::{http_request::{self, HttpRequest}, http_response::HttpResponse};

use crate::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};


pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match &req.method {
            http_request::Method::Get => match &req.resource {
                http_request::Resource::Path(s) => {
                    let route : Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_reponse(stream);
                        }
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_reponse(stream);
                        }
                    }
                }
            }
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_reponse(stream);
            }
        }
    }
}


use super::http::{Methods, Request, Response, StatusCode};
use super::server::Handler;

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Methods::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("<h1>ROOT</h1>".to_string())),
                "/hello" => Response::new(StatusCode::Ok, Some("<h1>Hello</h1>".to_string())),
                _ => Response::new(StatusCode::NotFound, None),
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}

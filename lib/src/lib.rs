extern crate actix_web;
use actix_web::HttpRequest;

pub fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

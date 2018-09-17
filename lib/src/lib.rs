extern crate actix_web;

use actix_web::{HttpRequest, Responder};
use std::cell::Cell;


#[derive(Debug, Default)]
pub struct AppState {
    pub counter: Cell<usize>,
}


pub fn index(request: &HttpRequest<AppState>) -> impl Responder {
    println!("counter is {}", request.state().counter.get());
    let count = request.state().counter.get() + 1;
    request.state().counter.set(count);
    format!("Request number is {}", count)
}

pub fn ping(_request: &HttpRequest) -> impl Responder {
    format!("pong")
}

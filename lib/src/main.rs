extern crate actix_web;
extern crate apate;

use actix_web::{server, App};
use apate::index;

fn main() {
    server::new( || App::new().resource("/", |r| r.f(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}

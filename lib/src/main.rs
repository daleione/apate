extern crate actix_web;
extern crate listenfd;

extern crate apate;

use actix_web::{server, App};
use listenfd::ListenFd;

use apate::index;

fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::new()
            .resource("/", |r| r.f(index))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:8088").unwrap()
    };

    server.run();
}

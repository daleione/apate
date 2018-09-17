extern crate actix_web;
extern crate listenfd;

extern crate apate;

use actix_web::{server, App, HttpResponse};
use listenfd::ListenFd;

use apate::{AppState, index, ping};

fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        vec![
            App::new()
                .resource("/ping", |r| r.f(ping))
                .boxed(),
            App::with_state(AppState::default())
                .prefix("/wechat")
                .resource("/test.html", |r| r.f(index))
                .boxed(),
            App::with_state(AppState::default())
                .prefix("/app1")
                .resource("/", |r| r.f(|_r| HttpResponse::Ok()))
                .boxed(),
        ]
    }).workers(1);

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("0.0.0.0:8088").unwrap()
    };

    server.run();
}

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod jobs;
mod server;

// run server
fn main() {
    server::run();
}

#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::path::{Path, PathBuf};
use rocket::response::{Redirect, NamedFile};

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

#[get("/")]
fn root() -> Redirect {
    Redirect::to("/index")
}

#[get("/index")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("../eshopper/index.html")).ok()
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("../eshopper/").join(file)).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![root, index, files]).launch();
}
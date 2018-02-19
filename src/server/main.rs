#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::io;
use std::path::{Path,PathBuf};

use rocket::response::NamedFile;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/api/test")]
fn test() -> &'static str {
    "Hello, world!"
}

#[get("/<file..>")]
fn files(file: PathBuf) -> NamedFile {
    match NamedFile::open(Path::new("static/").join(file)) {
        Result::Ok(val) => val,
        Result::Err(_err) =>
          NamedFile::open("static/index.html").unwrap()
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, test, files])
        .launch();
}

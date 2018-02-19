#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::io;
use std::path::PathBuf;

use rocket::response::NamedFile;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/api/test")]
fn test() -> &'static str {
    "Hello, world!"
}

#[get("/manifest.json")]
fn manifest() -> io::Result<NamedFile> {
    NamedFile::open("static/manifest.json")
}

#[get("/<_file..>")]
fn files(_file: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, files, manifest, test])
        .launch();
}

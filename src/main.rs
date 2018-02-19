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

#[get("/manifest.json")]
fn manifest() -> io::Result<NamedFile> {
    NamedFile::open("static/manifest.json")
}

#[get("/service-worker.js")]
fn sw() -> io::Result<NamedFile> {
    NamedFile::open("static/service-worker.js")
}

#[get("/images/<file..>")]
fn images(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/images/").join(file)).ok()
}

#[get("/js/<file..>")]
fn scripts(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/js/").join(file)).ok()
}

#[get("/<_file..>", rank = 2)]
fn files(_file: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, manifest, test, sw, images, scripts, files])
        .launch();
}

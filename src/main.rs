#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::serve::StaticFiles;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/static", StaticFiles::from("static"))
       // .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        //.mount("/static/assets", StaticFiles::from("static"))
        .launch();
}


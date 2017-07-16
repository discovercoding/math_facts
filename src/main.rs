#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rand;

mod gen_facts;
use gen_facts::generate_page;

use rocket::response::content;
use rocket::response::NamedFile;

use std::io;

#[derive(FromForm)]
struct Input {
    min: u32,
    max: u32,
    op: String,
    rows: usize,
    cols: usize,
}

fn validate(op: &str, min: u32, max: u32, num_rows: usize, num_cols: usize) -> Option<char> {
    let op = match op {
        "addition" => '+',
        "subtraction" => '-',
        "multiplication" => '×',
        "division" => '÷',
        _ => return None,
    };
    if max > 999 || min >= max {
        return None;
    }
    if num_rows > 25 || num_cols > 25 {
        return None;
    }
    Some(op)
}

#[get("/?<input>")]
fn generate_from_form(input: Input) -> Option<content::HTML<String>> {
    validate(&input.op, input.min, input.max, input.rows, input.cols).and_then(|op| {
        Some(content::HTML(generate_page(input.min, input.max, op, input.rows, input.cols)))
    })
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("index.html")
}

#[get("/style.css")]
fn style() -> io::Result<NamedFile> {
    NamedFile::open("style.css")
}

fn main() {
    rocket::ignite().mount(
        "/",
        routes![index, style, generate_from_form],
    ).launch();
}

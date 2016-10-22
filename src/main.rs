extern crate hyper;
extern crate select;

use hyper::Client;
use std::env;
use std::io::Read;
use select::document::Document;
use select::predicate::{Class, Name};

fn main() {
    let client = Client::new();
    let url = &env::args().nth(1).unwrap();
    let mut res =
        client.get(url)
              .send()
              .expect("Request failed");
    let mut body = String::new();
    res.read_to_string(&mut body).expect("Read failed");
    let document = Document::from(body.as_str());
    let rows = document.find(Class("a-row"));

    for row in rows.iter() {
        let title_node = row.find(Name("h5")).first();
        let price_node = row.find(Class("a-color-price")).first();
        if let (Some(title), Some(price)) = (title_node, price_node) {
            println!("* Book \"{}\", with price {}",
                title.text().trim(),
                price.text().trim()
            );
        }
    }
}

mod urls;

#[macro_use] extern crate nickel;

use nickel::{Nickel};


fn main() {
    let mut server = Nickel::new();

    urls::urls_definition(&mut server);

    server.listen("127.0.0.1:6767").unwrap();

}
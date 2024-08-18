#[macro_use] extern crate nickel;

use std::string;
use diesel::data_types::PgTimestamp;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use nickel::{HttpRouter, Nickel};

use matcha_lib::*;
use matcha_lib::models::user::User;
use matcha_lib::schema::*;

fn main() {
    let mut server = Nickel::new();

    // urls::urls_definition(&mut server);

    server.get(
        "**",
        nickel::middleware! { |req|
            println!("logging request: {:?}", req.origin.uri);
        },
    );
    server.utilize(router! {
        get "**" => |_req, _res| {
            test();
        }
    });

    server.listen("127.0.0.1:6767").unwrap();

}

fn test(){
    let conn = & mut establish_connection();

    create_post(conn, &String::from("Hello, world!"), &PgTimestamp(1234));


    let results= user::table
        .select(User::as_select())
        .load(conn)
        .expect("Error loading posts");



    println!("Displaying {} posts", results.len());
    for user_ in results {
        println!("{}", user_.name);
        println!("-----------");
        println!("{:?}\n", user_.birthdate);
    }
}
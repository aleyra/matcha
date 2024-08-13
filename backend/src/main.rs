mod urls;
mod data_types;

#[macro_use]
extern crate nickel;

use crate::data_types::User;

fn main() {
    // let mut server = Nickel::new();
    //
    // urls::urls_definition(&mut server);
    //
    // server.listen("127.0.0.1:6767").unwrap();


    let test = User::new();
    print!("Username = {:?}", test.username);

}

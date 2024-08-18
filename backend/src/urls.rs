// use nickel::{Nickel, HttpRouter};

// pub fn urls_definition(server: &mut Nickel) {
//     server.get(
//         "**",
//         nickel::middleware! { |req|
//             println!("logging request: {:?}", req.origin.uri);
//         },
//     );
//     server.utilize(router! {
//         get "**" => |_req, _res| {
//             println!("{:?}", _res.data());
//             "Hello world!"
//         }
//     });
// }

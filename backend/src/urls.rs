use nickel::{HttpRouter, Nickel};

pub fn urls_definition(server: &mut Nickel) {
    server.get(
        "**",
        middleware! { |req|
            println!("logging request: {:?}", req.origin.uri);
        },
    );
    server.utilize(router! {
        get "**" => |_req, _res| {
            println!("{:?}", _res.data());
            "Hello world!"
        }
    });
}

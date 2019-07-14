//! An example of serving static assets with Gotham.

extern crate gotham;

use gotham::handler::assets::FileOptions;
use gotham::router::builder::{build_simple_router, DefineSingleRoute, DrawRoutes};

pub fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Need to pass an arg which is the path to serve");
    let addr = "127.0.0.1:7878";
    println!(
        "Listening for requests at http://{} from path {:?}",
        addr, path
    );

    let mut default = String::new();
    default.push_str(&path);


    default += "/index.html";
    let router = build_simple_router(|route| {        
        //route.get("/").to_file(default);
        route.get("/*").to_dir(
            FileOptions::new(&path).build(),
        );
        route.get("/share").to_file(default);
    });

    

    gotham::start(addr, router)
}
//! An example of serving static assets with Gotham.

extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate hyper;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use gotham::handler::assets::FileOptions;
use gotham::router::builder::*;
use gotham::helpers::http::response::create_response;
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::single::single_pipeline;
use gotham::pipeline::single_middleware;
use gotham::state::{FromState, State};

use hyper::{Body, Response, StatusCode};

use std::sync::{Arc, Mutex};

use serde_json::json;

#[derive(Clone, StateData)]
struct RequestCounter {
    count: Arc<Mutex<usize>>,
}

/// Counter implementation.
impl RequestCounter {
    /// Creates a new request counter, setting the base state to `0`.
    fn new() -> Self {
        Self {
            count: Arc::new(Mutex::new(0)),
        }
    }

    /// Increments the internal counter state by `1`, and returns the
    /// new request counter as an atomic operation.
    fn incr(&self) -> usize {
        let mut w = self.count.lock().unwrap();
        *w += 1;
        *w
    }
}

fn get_apidata_handler(state: State) -> (State, Response<Body>) {
  let res = {
    let value = RequestCounter::borrow_from(&state).incr();
    let res = json!({
      "count": value
    });
    create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, res.to_string())
  };

  (state, res)
}

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

    let counter = RequestCounter::new();
    let middleware = StateMiddleware::new(counter);
    let pipeline = single_middleware(middleware);
    let (chain, pipelines) = single_pipeline(pipeline);

    default += "/index.html";
    let router = build_router(chain, pipelines, |route| {        
        //route.get("/").to_file(default);
        route.get("/*").to_dir(
            FileOptions::new(&path).build(),
        );
        route.get("/share").to_file(default);
        route.get("/api").to(get_apidata_handler);
    });

    

    gotham::start(addr, router)
}
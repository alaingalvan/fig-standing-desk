use nickel::{Nickel, JsonBody, HttpRouter, MediaType};

mod gpio;

#[derive(RustcDecodable, RustcEncodable)]
struct APIRequest {
    vector: f32,
    time: u32
}

/// Creates a Nickel HTTP Server
pub fn create_server() -> Nickel {

    let mut server = Nickel::new();

    let mut router = Nickel::router();

    router.post("/api", middleware! { |request, response|

      let data = request.json_as::<APIRequest>().unwrap();

      // Perform request with gpio device.
      gpio::send(data.vector, data.time);

      format!("")

    });

    server.utilize(router);

    server
}
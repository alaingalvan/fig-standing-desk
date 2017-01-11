extern crate iron;
extern crate bodyparser;

use iron::{Iron, Request, Response, status, IronResult};

fn start_server() {

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    }

    fn api_callback(req: &mut Request) -> IronResult<Response> {
        let body = req.get::<bodyparser::Raw>();
        match body {
            Ok(Some(body)) => println!("Read body:\n{}", body),
            Ok(None) => println!("No body"),
            Err(err) => println!("Error: {:?}", err),
        }

        let json_body = req.get::<bodyparser::Json>();
        match json_body {
            Ok(Some(json_body)) => println!("Parsed body:\n{:?}", json_body),
            Ok(None) => println!("No body"),
            Err(err) => println!("Error: {:?}", err),
        }

        Ok(Response::with(status::Ok))
    }

    let server = Iron::new(hello_world).http("localhost:74613").unwrap();



    println!("Started Fig Table Server @ Port 'table' (74613)");
}

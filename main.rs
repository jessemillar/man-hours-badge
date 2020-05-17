#![deny(warnings)]

use std::time::{SystemTime, UNIX_EPOCH};
use std::process::Command;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::collections::HashMap;

/// This is our service handler. It receives a Request, routes on its path, and returns a Future of a Response.
async fn man_hours(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Try POSTing data to /echo such as: `curl localhost:3000/echo -XPOST -d 'hello world'`",
        ))),

        // Calculate man hours from a repo
        (&Method::GET, "/hours") => {
            let query = req.uri().query();

            let params: HashMap<_, _> = url::form_urlencoded::parse(query.unwrap().as_bytes()).into_owned().collect();

            // Validate the request parameters, returning early if an invalid input is detected.
            let name = if let Some(n) = params.get("repo") {
                n
            } else {
                return Ok(Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .body("error".into())
                    .unwrap());
            };
            println!("{}", name);

            let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
            let repo_dir = &since_the_epoch.subsec_nanos().to_string();

            // Clone the repo
            Command::new("sh")
                    .arg("-c")
                    .arg(["git clone", name, "--no-checkout", repo_dir].join(" "))
                    .spawn()
                    .expect("Failed to clone repo");

            let git_log = Command::new("git log")
                    .current_dir(repo_dir)
                    .output()
                    .expect("git log command failed to start");

            Ok(Response::new(Body::from(git_log.stdout)))
        }

        // Return 404 otherwise
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([0, 0, 0, 0], 8080).into();
    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(man_hours)) });
    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}

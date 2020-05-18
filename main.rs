#![deny(warnings)]

use std::time::{SystemTime, UNIX_EPOCH};
use std::process::Command;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::collections::HashMap;
use regex::Regex;
use chrono::DateTime;

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
            // println!("{}", name);

            let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
            let repo_dir = ["repositories", &since_the_epoch.as_nanos().to_string()].join("/");

            // Clone the repo
            let git_log = Command::new("sh")
                    .arg("-c")
                    .arg(["git clone", name, "--no-checkout", &repo_dir, "&& cd", &repo_dir, "&& git log"].join(" "))
                    .output()
                    .expect("Failed to clone repo");

            let git_log_string = String::from_utf8_lossy(&git_log.stdout);
            let mut git_log_iterator = git_log_string.lines();

            let mut total_man_hours = chrono::Duration::hours(0);

            let re = Regex::new(r"^Date:\s+\w+\s\w+\s\d+\s\d+:\d+:\d+\s\d+\s.\d+$").unwrap();
            let mut previous_dt = DateTime::parse_from_str("Thu Jan 1 00:00:00 1970 +0000", "%a %b %d %H:%M:%S%.3f %Y %z");
            // Tue May 5 18:14:45 2015 -0600

            while let Some(line) = git_log_iterator.next() {
                // Parse out timestamps
                if re.is_match(line) {
                    let line = line.replace("Date:   ", "");
                    let dt = DateTime::parse_from_str(&line, "%a %b %d %H:%M:%S%.3f %Y %z");
                    let time_difference = previous_dt.unwrap()-dt.unwrap();
                    // println!("DIFFERENCE IN MINUTES {}", time_difference.num_minutes());
                    if time_difference > chrono::Duration::minutes(0) && time_difference < chrono::Duration::hours(10) {
                        // println!("Currently developing");
                        total_man_hours = total_man_hours + time_difference;
                    } else {
                        // println!("Starting a dev session");
                        total_man_hours = total_man_hours + chrono::Duration::minutes(30);
                    }
                    previous_dt = dt;
                    // println!("{}", line);
                    // println!("CURRENT TOTAL IN HOURS {}", total_man_hours.num_hours());
                }
            }

            Ok(Response::new(Body::from(total_man_hours.num_hours().to_string())))
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

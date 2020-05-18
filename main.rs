#![deny(warnings)]

use std::time::{SystemTime, UNIX_EPOCH};
use std::process::Command;
use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Method, Request, Response, Server, StatusCode};
use std::collections::HashMap;
use regex::Regex;
use chrono::DateTime;
use std::env;
use redis::Commands;
use std::thread;

extern crate redis;

type GenericError = Box<dyn std::error::Error + Send + Sync>;

fn calculate(redis_client: redis::Client, repo_name: String) {
    println!("Beginning calculation");

    // Calculate the man hours
    let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
    let repo_dir = ["repositories", &since_the_epoch.as_nanos().to_string()].join("/");

    // Clone the repo
    let git_log = Command::new("sh")
            .arg("-c")
            .arg(["git clone --bare", &repo_name, &repo_dir, "&& cd", &repo_dir, "&& git log"].join(" "))
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
            if time_difference > chrono::Duration::minutes(0) && time_difference < chrono::Duration::hours(8) {
                // println!("Currently developing");
                total_man_hours = total_man_hours + time_difference;
            } else {
                // println!("Starting a dev session");
                total_man_hours = total_man_hours + chrono::Duration::hours(1);
            }
            previous_dt = dt;
            // println!("{}", line);
            // println!("CURRENT TOTAL IN HOURS {}", total_man_hours.num_hours());
        }
    }

    let mut redis_connection = redis_client.get_connection().expect("Error creating Redis connection");
    let _ : () = redis_connection.set(repo_name, total_man_hours.num_hours()).expect("Error writing to Redis");

    println!("Finished calculation");
}

/// This is our service handler. It receives a Request, routes on its path, and returns a Future of a Response.
async fn man_hours(req: Request<Body>, redis_client: redis::Client) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Use the /health endpoint and pass ?repo= with an HTTP link for cloning your repo",
        ))),

        // Calculate man hours from a repo
        (&Method::GET, "/hours") => {
            let query = req.uri().query();
            let params: HashMap<_, _> = url::form_urlencoded::parse(query.unwrap().as_bytes()).into_owned().collect();

            // Validate the request parameters, returning early if an invalid input is detected.
            let repo_name = params.get("repo");
            if repo_name.is_none() {
                let json_response = "{
                    \"schemaVersion\": 1,
                    \"label\": \"man hours\",
                    \"message\": \"error\",
                    \"color\": \"critical\"
                }";

                let response = Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(json_response));
                return Ok(response.unwrap())
            };

            let mut redis_connection = redis_client.get_connection().expect("Error creating Redis connection");
            let redis_response: Option<String> = redis_connection.get(repo_name.unwrap().to_string()).expect("Error reading from Redis");
            let cached_man_hours = redis_response.unwrap_or_else(|| "calculating".to_string());
            println!("Cached hours: {}", cached_man_hours);

            let repo_name_string = repo_name.unwrap().to_string();

            // TODO Recalculate on missing or stale cache data
            if cached_man_hours == "calculating" {
                thread::spawn(move || {
                    calculate(redis_client, repo_name_string);
                });
            }

            // Return the cached value
            let json_response = ["{
                \"schemaVersion\": 1,
                \"label\": \"man hours\",
                \"message\": \"", &cached_man_hours, "\",
                \"color\": \"blueviolet\"
            }"].join("");

            let response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json_response));
            Ok(response.unwrap())
        }

        // Return 404 otherwise
        _ => {
            let not_found = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("404 not found"));
            Ok(not_found.unwrap())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let redis_client = redis::Client::open(env::var("REDIS_URL").unwrap())?;
    // let mut redis_connection = redis_client.get_connection()?;
    // let _ : () = redis_connection.set("poots", 69)?;
    // let poots: String = redis_connection.get("poots")?;
    // println!("Poots key: {}", poots);

    let addr = ([0, 0, 0, 0], env::var("PORT").unwrap().parse().unwrap()).into();
    let service = make_service_fn(move |_| {
        // Move a clone of `client` into the `service_fn`.
        let redis_client = redis_client.clone();
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                // Clone again to ensure that client outlives this closure.
                man_hours(req, redis_client.to_owned())
            }))
        }
    });
    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}

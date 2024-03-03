use reqwest::blocking;
use std::env;

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let url = env::var("WHM_URL")
        .map_err(|_| "Environment variable WHM_URL is not set. Please set it and try again.")?;

    let username = env::var("WHM_USERNAME").map_err(|_| {
        "Environment variable WHM_USERNAME is not set. Please set it and try again."
    })?;

    let secret = env::var("WHM_SECRET")
        .map_err(|_| "Environment variable WHM_SECRET is not set. Please set it and try again.")?;

    let outputfile = env::var("WHM_OUTPUTFILE")
        .map_err(|_| "Environment variable WHM_OUTPUTFILE is not set. Please set it and try again.")?;


    let error_file = env::var("WHM_LOGFILE")
        .map_err(|_| "Environment variable WHM_LOGFILE is not set. Please set it and try again.")?;

    let action = "GetProducts";
    let responsetype = "json";

    make_request(&url, &username, &secret, action, responsetype)?;

    Ok(())
}

fn make_request(
    url: &str,
    username: &str,
    password: &str,
    action: &str,
    responsetype: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = blocking::Client::new();

    let response = client
        .post(url)
        .form(&[
            ("action", action),
            ("username", username),
            ("password", password),
            ("responsetype", responsetype),
        ])
        .send()
        .map_err(|err| format!("Failed to send request: {}", err))?;

    if !response.status().is_success() {
        return Err(format!("Request failed with status code: {}", response.status()).into());
    }

    let body = response
        .text()
        .map_err(|err| format!("Failed to read response body: {}", err))?;

    println!("{:?}", body);

    Ok(())
}

use anyhow::{Context, Result};
use serde_json::Value;
use std::io::Read;
use std::time::Duration;

const MAX_RESPONSE_SIZE: u64 = 32 * 1024 * 1024;
const MAX_ATTEMPTS: usize = 3;

fn agent() -> ureq::Agent {
    ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(20))
        .build()
}

fn read_response(response: ureq::Response) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    response
        .into_reader()
        .take(MAX_RESPONSE_SIZE)
        .read_to_end(&mut bytes)
        .context("Failed to read HTTP response")?;
    Ok(bytes)
}

fn call_with_retries<F>(operation: &str, mut request: F) -> Result<ureq::Response>
where
    F: FnMut() -> std::result::Result<ureq::Response, Box<ureq::Error>>,
{
    let mut last_error = None;
    for attempt in 1..=MAX_ATTEMPTS {
        match request() {
            Ok(response) => return Ok(response),
            Err(error) => {
                last_error = Some(error);
                if attempt < MAX_ATTEMPTS {
                    std::thread::sleep(Duration::from_millis(200 * attempt as u64));
                }
            }
        }
    }
    Err(anyhow::anyhow!(
        "{operation} failed after {MAX_ATTEMPTS} attempts: {}",
        last_error.expect("retry loop records its final error")
    ))
}

pub fn get(url: &str, headers: &[(&str, &str)]) -> Result<Vec<u8>> {
    let response = call_with_retries("HTTP GET request", || {
        let agent = agent();
        let mut request = agent.get(url);
        for (name, value) in headers {
            request = request.set(name, value);
        }
        request.call().map_err(Box::new)
    })?;
    read_response(response)
}

pub fn get_query(url: &str, query: &[(&str, &str)], headers: &[(&str, &str)]) -> Result<Vec<u8>> {
    let response = call_with_retries("HTTP GET request", || {
        let agent = agent();
        let mut request = agent.get(url);
        for (name, value) in query {
            request = request.query(name, value);
        }
        for (name, value) in headers {
            request = request.set(name, value);
        }
        request.call().map_err(Box::new)
    })?;
    read_response(response)
}

pub fn post_json(url: &str, body: &Value, headers: &[(&str, &str)]) -> Result<Vec<u8>> {
    let response = call_with_retries("HTTP POST request", || {
        let agent = agent();
        let mut request = agent.post(url);
        for (name, value) in headers {
            request = request.set(name, value);
        }
        request.send_json(body).map_err(Box::new)
    })?;
    read_response(response)
}

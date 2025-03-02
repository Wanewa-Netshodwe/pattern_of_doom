use std::{
    io::{self, Error, Write},
    thread::{sleep, Thread},
    time::Duration,
};

use crate::models::{ContentInfo, Prompts, ReqBody};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;

async fn is_pattern_valid(mut query: String) -> Result<String, Box<dyn std::error::Error>> {
    const URL: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key=AIzaSyD0J0kSlm2t28l7vok4ydgtYkbWC9xgA6A";

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    query.push_str(" is this a mathmatical correct number sequence is it cohesive  if it is provide me a json response with the type of number pattern and diffuculity and general term formular  its its not  mark the valid filed in the json as invalid only provide the json no addition text is there in no description in the json 
    dont include ```json in the response but the object only 
    ");

    let req_body = ReqBody {
        contents: vec![ContentInfo {
            parts: vec![Prompts { text: query }],
        }],
    };

    let client = reqwest::Client::new();
    let response = client
        .post(URL)
        .headers(headers)
        .json(&req_body)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        eprintln!("API request failed: {}", error_text);
        return Err("API request failed".into());
    }

    let json_response: Value = response.json().await?;

    if let Some(text) = json_response
        .get("candidates")
        .and_then(|candidates| candidates[0].get("content"))
        .and_then(|content| content.get("parts"))
        .and_then(|parts| parts[0].get("text"))
        .and_then(|text| text.as_str())
    {
        return Ok(text.to_string());
        // println!("{}", text);
    } else {
        return Err("Could not find text in response".into());
    }
}

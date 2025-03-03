use crate::models::{ContentInfo, Prompts, ReqBody};
use dotenvy::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::{
    io::{self, Error, Write},
    thread::{sleep, Thread},
    time::Duration,
};

pub async fn is_pattern_valid(mut query: String) -> Result<String, Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("Key not found");
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",api_key);

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
        .post(url.as_str())
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
pub async fn give_hint(mut query: String) -> Result<String, Box<dyn std::error::Error>>  {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("Key not found");
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",api_key);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    query.push_str(" this a mathematical squence give me an hint that could be a usefull but  do not 
    give me a hint that will directly lead me to an answer
    ");

    let req_body = ReqBody {
        contents: vec![ContentInfo {
            parts: vec![Prompts { text: query }],
        }],
    };

    let client = reqwest::Client::new();
    let response = client
        .post(url.as_str())
        .headers(headers)
        .json(&req_body)
        .send()
        .await.unwrap();

    if !response.status().is_success() {
        let error_text = response.text().await?;
        eprintln!("API request failed: {}", error_text);
        return Err("API request failed".into());
    }

    let json_response: Value = response.json().await.unwrap();

    if let Some(text) = json_response
        .get("candidates")
        .and_then(|candidates| candidates[0].get("content"))
        .and_then(|content| content.get("parts"))
        .and_then(|parts| parts[0].get("text"))
        .and_then(|text| text.as_str())
    {
      
        return Ok(text.to_string());
       
    } else {
        return Err("Could not find text in response".into());
    }
}

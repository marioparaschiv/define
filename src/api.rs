use std::process;

use crate::types::DictionaryResponse;
use colored::Colorize;

pub async fn  perform_request(word: String) -> Result<DictionaryResponse, reqwest::Error> {
	let client = reqwest::Client::new();

    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);

    let _request = client.get(url).send().await;

	if _request.is_err() {
		return Err(_request.err().unwrap());
	}

	let request = _request.unwrap();
	let status = request.status();

	if status == 404 {
		println!("{}", "No definition found.".red().bold());
		process::exit(1);
	}

	if status == 429 {
		let retry_after = request.headers().get("retry-after").unwrap().to_str().unwrap();
		let ratelimited = format!("You are currently ratelimited. Please try again in {} second(s).", retry_after).red().bold();

		println!("{}", ratelimited);
		process::exit(1);
	}


	let body = request.text().await.unwrap();

	if status != 200 {
		let unknown_response = format!("Got unknown response: {}: {}", status, body).red().bold();

		println!("{}", unknown_response);
		process::exit(1);
	}

	let json = parse_response(body);

	Ok(json)
}

pub fn parse_response(body: String) -> DictionaryResponse {
	let json: Result<DictionaryResponse, serde_json::Error> = serde_json::from_str(&body);

	if !json.is_ok() {
		println!("Ran into an error while decoding API response: {}", json.err().unwrap());
		std::process::exit(1);
	}

	json.unwrap()
}
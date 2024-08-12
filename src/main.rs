mod format;
mod types;
mod api;

use std::{borrow::Borrow, process};
use clap::Parser;
use tokio;

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// Define words right from your terminal.
pub struct Arguments {
    word: String,

	/// Show synonyms
	#[arg(long, short, action)]
	synonyms: bool,

	/// Show examples
	#[arg(long, short, action)]
	examples: bool,

	/// Show antonyms
	#[arg(long, short, action)]
	antonyms: bool,

	/// Show synonyms, antonyms, and examples
	#[arg(long, short, action)]
	full: bool
}

#[tokio::main]
async fn main() {
    let binding = Arguments::parse();
    let args = binding.borrow();

	let items = api::perform_request(args.word.to_string()).await;

	if items.is_err() {
		println!("Failed to look up definition for word: {}", items.err().unwrap());
		process::exit(1);
	}

	for item in items.unwrap().iter() {
		println!("{}", format::format_item(args, item));
	}
}

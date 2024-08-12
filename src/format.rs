use crate::{types::{Definition, Word, Meaning}, Arguments};
use colored::{ColoredString, Colorize};

#[allow(unused_variables)]
pub fn format_definition(args: &Arguments, definition: &Definition) -> String {
    let mut value = format!("• {}", definition.definition);
	let example = definition.example.as_deref().unwrap_or("".into());
	let string: String = format!("\n\"{}\"", example.to_string());

	if (args.examples|| args.full) && example.len() != 0 {
		let colored = string.bright_black().italic().to_string();
		value.push_str(&colored);
	}

	return value;
}

#[allow(unused_variables)]
pub fn format_definitions(args: &Arguments, definitions: &Vec<Definition>) -> String {
    definitions
        .iter()
        .map(|definition| format_definition(args, definition))
        .collect::<Vec<String>>()
        .join("\n".into())
}

#[allow(unused_variables)]
pub fn format_meaning(args: &Arguments, meaning: &Meaning) -> String {
    let mut definitions = format_definitions(args, &meaning.definitions);

	let synonyms: &str = if (args.synonyms || args.full) && meaning.synonyms.len() != 0 {
		let synonyms = meaning.synonyms.join(", ");

		&format!(" ({})", synonyms).bright_black().italic().to_string()
	} else {
		""
	};

	let antonyms: &str = if (args.antonyms || args.full) && meaning.antonyms.len() != 0 {
		let antonyms = meaning.antonyms.join(", ");

		&format!(" ({})", antonyms).bright_red().bold().to_string()
	} else {
		""
	};

    definitions.insert_str(
        0,
        &format!("{}{}{}\n", meaning.part_of_speech.bright_purple().italic(), synonyms, antonyms),
    );

    definitions
}

#[allow(unused_variables)]
pub fn format_meanings(args: &Arguments, meanings: &Vec<Meaning>) -> String {
    meanings
        .iter()
        .map(|meaning| format_meaning(args, meaning))
        .collect::<Vec<String>>()
        .join("\n\n".into())
}

#[allow(unused_variables)]
pub fn format_phonetic(args: &Arguments, phonetic: String) -> ColoredString {
	phonetic.bright_blue().italic()
}

#[allow(unused_variables)]
pub fn format_word(args: &Arguments, word: String) -> ColoredString {
    word.bright_green().bold()
}

#[allow(unused_variables)]
pub fn format_item(args: &Arguments, item: &Word) -> String {
	format!(
		"\n{word} {phonetic}\n{meanings}\n\n",
		word = format_word(args, item.word.to_string()),
		phonetic = format_phonetic(args, item.phonetic.as_deref().unwrap_or("".into()).to_string()),
		meanings = format_meanings(args, &item.meanings)
	)
}

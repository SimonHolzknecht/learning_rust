pub fn convert(raw_input: &String) -> String {
    let mut pig_latin = String::new();
    let word_amount =raw_input.split_ascii_whitespace().count();
    for (index, word) in raw_input.split_ascii_whitespace().enumerate() {
        pig_latin += &convert_word(word);
        if index != word_amount - 1 {
            pig_latin += " ";
        }
    }
    pig_latin
}

fn convert_word(word: &str) -> String {
    match word.chars().next().expect("Could not retrieve the first char of the word") {
        'a' | 'e' | 'i' | 'o' | 'u' => convert_vowel_word(word),
        _ => convert_consonant_word(word)
    }
}

fn convert_vowel_word(word: &str) -> String {
    format!("{}-hay", word)
}

fn convert_consonant_word(word: &str) -> String {
    let mut pig_latin_word = String::from(word);
    let first_letter = pig_latin_word.remove(0);
    format!("{}-{}ay", pig_latin_word, first_letter)
}

pub fn encrypt(word: &String) -> String {
    word.chars()
        .map(|character| match character {
            ' ' => ' ',
            _ => '-',
        })
        .collect()
}

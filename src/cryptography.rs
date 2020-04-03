pub fn encrypt(word: &str) -> String {
    word.replace(|character: char| character.is_alphabetic(), "-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        assert_eq!("----", encrypt("Word"));
        assert_eq!("-- -----", encrypt("is truly"));
        assert_eq!(" ---- ", encrypt(" well "));
        assert_eq!("---------!", encrypt("encrypted!"));
    }
}

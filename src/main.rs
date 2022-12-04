use std::collections::HashMap;

#[rustfmt::skip]
fn translate(rune: char) -> String {
    let futhark_map: HashMap<char, char> = HashMap::from_iter([
        ('ᚠ', 'f'), ('ᚢ', 'u'), ('ᚦ', 'þ'), ('ᚨ', 'a'), ('ᚱ', 'r'),
        ('ᚲ', 'k'), ('ᚷ', 'g'), ('ᚹ', 'w'), ('ᚺ', 'h'), ('ᚻ', 'h'),
        ('ᚾ', 'n'), ('ᛁ', 'i'), ('ᛃ', 'j'), ('ᛇ', 'æ'), ('ᛈ', 'p'),
        ('ᛉ', 'z'), ('ᛊ', 's'), ('ᛋ', 's'), ('ᛏ', 't'), ('ᛒ', 'b'),
        ('ᛖ', 'e'), ('ᛗ', 'm'), ('ᛚ', 'l'), ('ᛜ', 'ŋ'), ('ᛟ', 'o'),
        ('ᛞ', 'd'),
    ]);

    // Find the Latin character corresponding to the given Futhark rune.
    // If no such character exists, return the rune itself.
    futhark_map
        .iter()
        .find(|(f, _)| *f == &rune)
        .map_or_else(|| rune.to_string(), |(_, l)| l.to_string())
}

fn main() {
    const HELP_TEXT: &str = r#"Usage: futhark2lating -f <runes>
  Converts a string of Futhark runes to Latin letters

  -f, --futhark <futhark>   the string of Futhark runes to convert
  -h, --help                display this help and exit"#;

    if let Some(futhark) = std::env::args()
        .skip(1)
        .find(|arg| arg == "-f" || arg == "--futhark")
    {
        // Collect the string of Futhark runes that follows the "-f" flag.
        let runes: String = std::env::args()
            .skip_while(|arg| arg != &futhark)
            .skip(1)
            .collect();

        // Cconvert each Futhark rune to its corresponding Latin letter.
        let latin: String = runes
            .split_whitespace()
            .map(|substr| substr.chars().map(translate).collect::<String>())
            .collect::<String>();

        println!("{}", latin);
    } else {
        println!("{}", HELP_TEXT);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translation_test() {
        assert_eq!(translate('ᚠ'), "f");
        assert_eq!(translate('ᚢ'), "u");
        assert_eq!(translate('ᚦ'), "þ");
        assert_eq!(translate('ᚨ'), "a");
        assert_eq!(translate('ᚱ'), "r");
        assert_eq!(translate('ᚲ'), "k");
        assert_eq!(translate('ᚷ'), "g");
        assert_eq!(translate('ᚹ'), "w");
        assert_eq!(translate('ᚺ'), "h");
        assert_eq!(translate('ᚻ'), "h");
        assert_eq!(translate('ᚾ'), "n");
        assert_eq!(translate('ᛁ'), "i");
        assert_eq!(translate('ᛃ'), "j");
        assert_eq!(translate('ᛇ'), "æ");
        assert_eq!(translate('ᛈ'), "p");
        assert_eq!(translate('ᛉ'), "z");
        assert_eq!(translate('ᛊ'), "s");
        assert_eq!(translate('ᛋ'), "s");
        assert_eq!(translate('ᛏ'), "t");
        assert_eq!(translate('ᛒ'), "b");
        assert_eq!(translate('ᛖ'), "e");
        assert_eq!(translate('ᛗ'), "m");
        assert_eq!(translate('ᛚ'), "l");
        assert_eq!(translate('ᛜ'), "ŋ");
        assert_eq!(translate('ᛟ'), "o");
        assert_eq!(translate('ᛞ'), "d");
        assert_eq!(translate('?'), "?");
    }
}

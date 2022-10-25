fn futhark_to_latin(rune: char) -> char {
    match rune {
        'ᚠ' => 'f',
        'ᚢ' => 'u',
        'ᚦ' => 'þ',
        'ᚨ' => 'a',
        'ᚱ' => 'r',
        'ᚲ' => 'k',
        'ᚷ' => 'g',
        'ᚹ' => 'w',
        'ᚺ' => 'h',
        'ᚻ' => 'h',
        'ᚾ' => 'n',
        'ᛁ' => 'i',
        'ᛃ' => 'j',
        'ᛇ' => 'æ',
        'ᛈ' => 'p',
        'ᛉ' => 'z',
        'ᛊ' => 's',
        'ᛋ' => 's',
        'ᛏ' => 't',
        'ᛒ' => 'b',
        'ᛖ' => 'e',
        'ᛗ' => 'm',
        'ᛚ' => 'l',
        'ᛜ' => 'ŋ',
        'ᛟ' => 'o',
        'ᛞ' => 'd',
        '-' => ' ',

        rune => rune,
    }
}

fn latin_to_futhark(script: char) -> char {
    match script {
        'f' => 'ᚠ',
        'u' => 'ᚢ',
        'þ' => 'ᚦ',
        'a' => 'ᚨ',
        'r' => 'ᚱ',
        'k' => 'ᚲ',
        'g' => 'ᚷ',
        'w' => 'ᚹ',
        'h' => 'ᚺ',
        'n' => 'ᚾ',
        'i' => 'ᛁ',
        'j' => 'ᛃ',
        'æ' => 'ᛇ',
        'p' => 'ᛈ',
        'z' => 'ᛉ',
        's' => 'ᛊ',
        't' => 'ᛏ',
        'b' => 'ᛒ',
        'e' => 'ᛖ',
        'm' => 'ᛗ',
        'l' => 'ᛚ',
        'ŋ' => 'ᛜ',
        'o' => 'ᛟ',
        'd' => 'ᛞ',
        '-' => ' ',

        script => script,
    }
}

fn main() {
    let mut args = std::env::args();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-f" | "--futhark" => {
                if let Some(futhark) = args.next() {
                    println!(
                        "{}",
                        futhark.chars().map(futhark_to_latin).collect::<String>()
                    )
                } else {
                    panic!("Add at least one argument");
                }
            }
            "-l" | "--latin" => {
                if let Some(latin) = args.next() {
                    println!(
                        "{}",
                        latin.chars().map(latin_to_futhark).collect::<String>()
                    )
                } else {
                    panic!("Add at least one argument");
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        todo!("Add test suite!")
    }
}

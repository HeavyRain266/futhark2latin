#[path = "../include/mappings.rs"] mod mappings;

use mappings::{futhark_to_latin, latin_to_futhark};

fn main() {
    let mut args = std::env::args();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-f" | "--futhark" => {
                if let Some(futhark) = args.next() {
                    println!("{}", futhark.chars().map(futhark_to_latin).collect::<String>())
                } else {
                    panic!("Add at least one argument");
                }
            }
            "-l" | "--latin" => {
                if let Some(latin) = args.next() {
                    println!("{}", latin.chars().map(latin_to_futhark).collect::<String>())
                } else {
                    panic!("Add at least one argument");
                }
            }
            _ => {}
        }
    }
}

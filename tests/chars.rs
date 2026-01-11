use parsy::{Parser, helpers::char};

#[test]
fn longer_chars() {
    let parser = char('a');

    let err = parser.parse_str("à").unwrap_err();

    println!("{err:?}");
}

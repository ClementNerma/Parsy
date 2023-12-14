use crate::{atoms::whitespace, chainings::DebugType, parser::Parser, parsers::*};

#[test]
pub fn complex_test() {
    let parser = just("Hello")
        .repeated()
        .at_least(1)
        .at_most(1)
        .then(just(" "))
        .then(choice((just("World"), just("world"))))
        .then(lookahead(char('!')))
        .followed_by(just("!"))
        .then(filter(|c| c == '!'))
        .then(whitespaces())
        .then(end())
        .spanned()
        .full();

    let input = "Hello world!\n";

    let parsed = parser.parse_str(input).unwrap();

    assert_eq!(parsed.data.at.start.offset(), 0);
    assert_eq!(parsed.data.at.len, input.len());
}

#[test]
pub fn recursive_test() {
    let parser_1212 =
        recursive(|number| just("1").then(just("2")).then(number.or_not().map(|_| ()))).full();

    parser_1212.parse_str("12").unwrap();

    parser_1212.parse_str("1212").unwrap();

    parser_1212.parse_str("121212").unwrap();

    parser_1212.parse_str("12121212").unwrap();

    parser_1212.parse_str("1").err().unwrap();
}

#[test]
pub fn late_test() {
    let a = late();

    let ba = a.clone().then(char('b')).then(a.clone());

    a.finish(char('a'));

    ba.parse_str("aba").unwrap();
}

#[test]
pub fn utf8() {
    let parser = just("é").then(whitespace()).then(char('è')).full();

    parser.parse_str("é è").unwrap();
    parser.parse_str("e è").err().unwrap();
    parser.parse_str("é e").err().unwrap();
    parser.parse_str("e e").err().unwrap();
}

#[test]
pub fn utf8_boundaries() {
    let parser = char('é').repeated().at_least(1).collect_string();

    let token = parser.parse_str("é").unwrap();

    assert_eq!(token.at.len, 'é'.len_utf8());
}

#[allow(dead_code)]
fn simple_debug<T>(debug: DebugType<'_, '_, T>) {
    match debug {
        DebugType::Input(input) => println!("{input:#?}"),
        DebugType::Result(result) => match result {
            Ok(ok) => println!("{:#?}", ok.at),
            Err(err) => println!("{err:#?}"),
        },
    }
}

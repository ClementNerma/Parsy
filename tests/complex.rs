use std::sync::LazyLock;

use parsy::{Parser, atoms::whitespace, chainings::DebugType, parsers::*};

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
    let parser =
        recursive(|number| just("1").then(just("2")).then(number.or_not().map(|_| ()))).full();

    let parser_shared =
        recursive_shared(|number| just("1").then(just("2")).then(number.or_not().map(|_| ())))
            .full();

    for input in ["12", "1212", "121212", "12121212"] {
        parser.parse_str(input).unwrap();
        parser_shared.parse_str(input).unwrap();
        PARSER_SHARED.parse_str(input).unwrap();
    }

    for input in ["1", "2", "21", "22", "3", "13", "23", "123"] {
        parser.parse_str(input).err().unwrap();
        parser_shared.parse_str(input).err().unwrap();
        PARSER_SHARED.parse_str(input).err().unwrap();
    }
}

#[test]
pub fn late_test() {
    let a = to_define();

    let ba = a.clone().then(char('b')).then(a.clone());

    a.define(char('a'));

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

static PARSER_SHARED: LazyLock<Box<dyn Parser<()> + Send + Sync + 'static>> = LazyLock::new(|| {
    Box::new(
        recursive_shared(|number| just("1").then(just("2")).then(number.or_not().map(|_| ())))
            .full()
            .map(|_| ()),
    )
});

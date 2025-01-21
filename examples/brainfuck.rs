use parsy::{char, choice, Parser};

fn main() {
    let bf_parser = parser();

    println!("Parser output #1: {:#?}", bf_parser.parse_str("><+-.,[]"));
    println!(
        "Parsed data #1: {:#?}",
        bf_parser.parse_str("><+-.,[]").unwrap().data.0
    );
    println!(
        "Parser output #2: {:#?}",
        bf_parser.parse_str("INVALID BRAINFUCK CODE")
    );
}

fn parser() -> impl Parser<Program> {
    let instruction = choice((
        char('>').to(Instruction::NextCell),
        char('<').to(Instruction::PrevCell),
        char('+').to(Instruction::Inc),
        char('-').to(Instruction::Dec),
        char('.').to(Instruction::Output),
        char(',').to(Instruction::Input),
        char('[').to(Instruction::BeginLoop),
        char(']').to(Instruction::EndLoop),
    ));

    instruction.repeated_into_vec().full().map(Program)
}

#[derive(Debug)]
struct Program(Vec<Instruction>);

#[derive(Debug, Clone, Copy)]
enum Instruction {
    NextCell,  // >
    PrevCell,  // <
    Inc,       // +
    Dec,       // -
    Output,    // .
    Input,     // ,
    BeginLoop, // [
    EndLoop,   // ]
}

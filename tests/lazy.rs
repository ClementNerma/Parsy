use parsy::{
    ParserConstUtils,
    parsers::{LazilyDefined, helpers::lazily_define},
};

static _A: LazilyDefined<()> = lazily_define(|| Box::new(_B.static_ref().repeated().to(())));
static _B: LazilyDefined<()> = lazily_define(|| Box::new(_A.static_ref().repeated().to(())));

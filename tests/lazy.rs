use parsy::{Parser, helpers::lazily_defined, timed::LazilyDefined};

static _A: LazilyDefined<()> = lazily_defined(|| Box::new(_B.static_ref().repeated().to(())));
static _B: LazilyDefined<()> = lazily_defined(|| Box::new(_A.static_ref().repeated().to(())));

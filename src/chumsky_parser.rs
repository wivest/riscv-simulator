use chumsky::prelude::*;

fn register<'src>() -> impl Parser<'src, &'src str, &'src str> {
    let register = just("x").ignore_then(text::int(10)).map(|idx| idx);
    register
}


use super::StrInfo;

mod ctx {
    #[derive(Default)]
    pub struct PosCtx {
        pub row: usize,
        pub col: usize,
    } 
}

/// Positional info in a string
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}

impl<'a> StrInfo<'a> for Pos {
    type Context = ctx::PosCtx;

    fn generate(ctx: &mut Self::Context, mut ts: &'a str) -> Self {
        let row = ctx.row;
        let col = ctx.col;
        while let Some(pos) = ts.find("\n") {
            ts = &ts[pos + 1..];
            ctx.row += 1;
            ctx.col = 0;
        }
        ctx.col += ts.len();
        Self { row, col }
    }
}

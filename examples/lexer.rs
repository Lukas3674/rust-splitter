
use core::ops::Range;
use splitter::{StrInfo, StrSplitter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TType {
    Whitespace,
    Bool,
    Operator,
    Idenifier,
    Unknown,
}

impl<'a> StrInfo<'a> for TType {
    type Context = ();
    fn generate(_ctx: &mut Self::Context, ts: &'a str) -> Self {
        match ts {
            " " => TType::Whitespace,
            "true" | "false" => TType::Bool,
            "||" | "&&" => TType::Operator,
            ts if ts.chars().all(char::is_alphabetic) => {
                TType::Idenifier
            }
            _ => TType::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, StrInfo)]
struct Token<'a> {
    content: &'a str,
    token_type: TType,
    span: Range<usize>,
}

fn main() {
    use TType::*;

    let sp = StrSplitter::new(
        "true || yep && false && unknown5",
        " ",
    ).with_info::<Token>();

    let tokens = sp.collect::<Vec<_>>();

    assert_eq!(tokens, vec![
        Token { content: "true", token_type: Bool, span: 0..4 },
        Token { content: " ", token_type: Whitespace, span: 4..5 },
        Token { content: "||", token_type: Operator, span: 5..7 },
        Token { content: " ", token_type: Whitespace, span: 7..8 },
        Token { content: "yep", token_type: Idenifier, span: 8..11 },
        Token { content: " ", token_type: Whitespace, span: 11..12 },
        Token { content: "&&", token_type: Operator, span: 12..14 },
        Token { content: " ", token_type: Whitespace, span: 14..15 },
        Token { content: "false", token_type: Bool, span: 15..20 },
        Token { content: " ", token_type: Whitespace, span: 20..21 },
        Token { content: "&&", token_type: Operator, span: 21..23 },
        Token { content: " ", token_type: Whitespace, span: 23..24 },
        Token { content: "unknown5", token_type: Unknown, span: 24..32 },
    ])
}
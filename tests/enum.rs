
use splitter::{StrSplitter, StrInfo};

#[derive(Debug, PartialEq, StrInfo)]
enum StrEnum {
    #[splitter(" " | "\n")]
    WS,

    #[splitter(":)")]
    Smiley,

    #[splitter(s if s.chars().all(char::is_numeric))]
    Number,

    #[splitter(_)]
    Other,
}

#[test]
fn string() {
    let sp = StrSplitter::new("hello\nsplitter :) 15", [" ", "\n"]).with_info::<StrEnum>();
    assert_eq!(
        sp.collect::<Vec<_>>(),
        vec![
            StrEnum::Other,
            StrEnum::WS,
            StrEnum::Other,
            StrEnum::WS,
            StrEnum::Smiley,
            StrEnum::WS,
            StrEnum::Number,
        ],
    );
}

use splitter::{Splitter, Info};

#[derive(Debug, PartialEq, Info)]
#[splitter(u8)]
enum SliceEnum {
    #[splitter(b" " | b"\n")]
    WS,

    #[splitter(b":)")]
    Smiley,

    #[splitter(bs if bs == b"15")]
    Number,

    #[splitter(_)]
    Other,
}

#[test]
fn slice() {
    let sp = Splitter::new(b"hello\nsplitter :) 15", [b" ", b"\n"]).with_info::<SliceEnum>();
    assert_eq!(
        sp.collect::<Vec<_>>(),
        vec![
            SliceEnum::Other,
            SliceEnum::WS,
            SliceEnum::Other,
            SliceEnum::WS,
            SliceEnum::Smiley,
            SliceEnum::WS,
            SliceEnum::Number,
        ],
    );
}

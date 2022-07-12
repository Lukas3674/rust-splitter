
fn main() {
    slice::run();
    string::run();
}

mod slice {
    use splitter::{Splitter, Info};

    pub fn run() {
        fn is_number(ts: &[u8]) -> bool {
            ts == b"15"
        }

        //
        // A enum with the 'Info'-derive can't have variants with fields
        // 
        // Every enum-variant needs a #[splitter(...)] attribute
        //
        // The attribute must contain a &[T] pattern (similar to match)
        //
        // The pattern can consist of a '|' seperated list of &[T], a variable name, or '_',
        // followed by a optional if statement
        //
        // The Info will use the first match, so ordering is important
        //

        #[derive(Debug, PartialEq, Info)]
        #[splitter(u8)]
        enum SliceEnum {
            #[splitter(b" " | b"\n")]
            WS,

            #[splitter(b":)")]
            Smiley,

            #[splitter(s if is_number(s))]
            Number,

            #[splitter(_)]
            Other,
        }

        let sp = Splitter::new(
            b"hello\nsplitter :) 15",
            [" ", "\n"],
        ).with_info::<SliceEnum>();

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
}

mod string {
    use splitter::{StrSplitter, StrInfo};

    pub fn run() {
        fn is_number(s: &str) -> bool {
            // can do some regex here if you want
            s.chars().all(char::is_numeric)
        }

        //
        // A enum with the 'StrInfo'-derive can't have variants with fields
        // 
        // Every enum-variant needs a #[splitter(...)] attribute
        //
        // The attribute must contain a &str pattern (similar to match)
        //
        // The pattern can consist of a '|' seperated list of &str, a variable name, or '_',
        // followed by a optional if statement
        //
        // The StrInfo will use the first match, so ordering is important
        //

        #[derive(Debug, PartialEq, StrInfo)]
        enum StrEnum {
            #[splitter(" " | "\n")]
            WS,

            #[splitter(":)")]
            Smiley,

            #[splitter(s if is_number(s))]
            Number,

            #[splitter(_)]
            Other,
        }

        let sp = StrSplitter::new(
            "hello\nsplitter :) 15",
            [" ", "\n"],
        ).with_info::<StrEnum>();

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
}

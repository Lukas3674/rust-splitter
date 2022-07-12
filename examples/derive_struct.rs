
fn main() {
    slice::run();
    string::run();
}

mod slice {
    use std::ops::Range;
    use splitter::{Info, Splitter};

    pub fn run() {
        #[derive(Debug, PartialEq, Info)]
        struct Custom<'a, T> {
            content: &'a [T],
            span: Range<usize>,
        }

        let sp = Splitter::new(b"bytes example", b" ").with_info::<Custom<u8>>();
        assert_eq!(
            sp.collect::<Vec<_>>(),
            vec![
                Custom { content: b"bytes", span: 0..5 },
                Custom { content: b" ", span: 5..6 },
                Custom { content: b"example", span: 6..13 },
            ],
        );

        #[derive(Debug, PartialEq, Info)]
        // it is also possible to define the type of the slice
        #[splitter(u8)]
        struct CustomU8<'a> {
            content: &'a [u8],
            // this uses the 'impls' feature, which aautomatically implements `Info`
            // for usefull types from `core` and `std`, see README.md for more details
            span: Range<usize>,
        }

        let sp = Splitter::new(b"bytes example", b" ").with_info::<CustomU8>();
        assert_eq!(
            sp.collect::<Vec<_>>(),
            vec![
                CustomU8 { content: b"bytes", span: 0..5 },
                CustomU8 { content: b" ", span: 5..6 },
                CustomU8 { content: b"example", span: 6..13 },
            ],
        );
    }
}

mod string {
    use std::ops::Range;
    use splitter::{StrInfo, StrSplitter};

    pub fn run() {
        #[derive(Debug, PartialEq, StrInfo)]
        struct Custom<'a> {
            content: &'a str,
            // this uses the 'impls' feature, which aautomatically implements `StrInfo`
            // for usefull types from `core` and `std`, see README.md for more details
            span: Range<usize>,
        }
        
        let sp = StrSplitter::new("bytes example", " ").with_info::<Custom>();
        assert_eq!(
            sp.collect::<Vec<_>>(),
            vec![
                Custom { content: "bytes", span: 0..5 },
                Custom { content: " ", span: 5..6 },
                Custom { content: "example", span: 6..13 },
            ],
        );
    }
}

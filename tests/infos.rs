
use splitter::{Splitter, StrSplitter};

#[test]
fn span() {
    use splitter::info::span::Span;
    let sp = Splitter::new(b"12534", b"5").with_info::<Span>();
    assert_eq!(
        sp.collect::<Vec<_>>(),
        vec![
            Span { start: 0, end: 2 },
            Span { start: 2, end: 3 },
            Span { start: 3, end: 5 },
        ],
    );
}

#[test]
fn str_span() {
    use splitter::str_info::span::Span;
    let sp = StrSplitter::new("12534", "5").with_info::<Span>();
    assert_eq!(
        sp.collect::<Vec<_>>(),
        vec![
            Span { start: 0, end: 2 },
            Span { start: 2, end: 3 },
            Span { start: 3, end: 5 },
        ],
    );
}

#[test]
fn str_pos() {
    use splitter::str_info::pos::Pos;
    let sp = StrSplitter::new("12\n534\n657\n", "5").with_info::<Pos>();
    assert_eq!(
        sp.collect::<Vec<_>>(),
        vec![
            Pos { row: 0, col: 0 },
            Pos { row: 1, col: 0 },
            Pos { row: 1, col: 1 },
            Pos { row: 2, col: 1 },
            Pos { row: 2, col: 2 },
        ],
    );
}

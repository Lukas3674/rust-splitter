
use splitter::StrSplitter;

fn main() {
    let sp = StrSplitter::new("bytes example", " ");
    assert_eq!(
        sp.collect::<Vec<_>>(),
        vec!["bytes", " ", "example"],
    );

    let sp = StrSplitter::new("123", "");
    assert_eq!(
        sp.collect::<Vec<_>>(),
        vec!["1", "2", "3"],
    );

    let sp = StrSplitter::new("bytes example", ["bytes", "example"]);
    assert_eq!(
        sp.collect::<Vec<_>>(),
        vec!["bytes", " ", "example"],
    );
}


use splitter::StrSplitter;

#[test]
fn basic() {
    let sp = StrSplitter::new("12334", ["2", "4"]);
    let re: Vec<&str> = vec!["1", "2", "33", "4"];
    assert_eq!(sp.collect::<Vec<_>>(), re);
}

#[test]
fn empty_inp() {
    let sp = StrSplitter::new("", ["1"]);
    let re: Vec<&str> = vec![];
    assert_eq!(sp.collect::<Vec<_>>(), re);
}

#[test]
fn empty_sep() {
    let sp = StrSplitter::new("123", "");
    let re: Vec<&str> = vec!["1", "2", "3"];
    assert_eq!(sp.collect::<Vec<_>>(), re);
}

#[test]
fn char() {
    let sp = StrSplitter::new("123", '1');
    let re: Vec<&str> = vec!["1", "23"];
    assert_eq!(sp.collect::<Vec<_>>(), re);
}

#[test]
fn multiple() {
    let sp = StrSplitter::new("123334", "33");
    let re: Vec<&str> = vec!["12", "33", "34"];
    assert_eq!(sp.collect::<Vec<_>>(), re);
}

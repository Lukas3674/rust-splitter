
use splitter::Splitter;

#[test]
fn basic() {
    let sp = Splitter::new(&[1, 2, 3, 3, 4], [[2], [4]]);
    let re: Vec<&[usize]> = vec![&[1], &[2], &[3, 3], &[4]];
    assert_eq!(sp.collect::<Vec<_>>(), re);
}

#[test]
fn basic_bytes() {
    let sp = Splitter::new(b"12334", [b"2", b"4"]);
    let re: Vec<&[u8]> = vec![b"1", b"2", b"33", b"4"];
    assert_eq!(sp.collect::<Vec<_>>(), re);
}

#[test]
fn empty_inp() {
    let sp = Splitter::new(b"", [b"1"]);
    let re: Vec<&[u8]> = vec![];
    assert_eq!(sp.collect::<Vec<_>>(), re);
}

#[test]
fn empty_sep() {
    let sp = Splitter::new(b"123", b"");
    let re: Vec<&[u8]> = vec![b"1", b"2", b"3"];
    assert_eq!(sp.collect::<Vec<_>>(), re);
}

#[test]
fn byte() {
    let sp = Splitter::new(b"123", b'1');
    let re: Vec<&[u8]> = vec![b"1", b"23"];
    assert_eq!(sp.collect::<Vec<_>>(), re);
}

#[test]
fn multiple() {
    let sp = Splitter::new(b"123334", b"33");
    let re: Vec<&[u8]> = vec![b"12", b"33", b"34"];
    assert_eq!(sp.collect::<Vec<_>>(), re);
}

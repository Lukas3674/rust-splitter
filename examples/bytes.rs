
use splitter::Splitter;

fn main() {
    let sp = Splitter::new(b"bytes example", b" ");
    let re: Vec<&[u8]> = vec![b"bytes", b" ", b"example"];
    assert_eq!(sp.collect::<Vec<_>>(), re);

    let sp = Splitter::new(b"123", b"");
    let re: Vec<&[u8]> = vec![b"1", b"2", b"3"];
    assert_eq!(sp.collect::<Vec<_>>(), re);

    let s: [&[u8]; 2] = [b"bytes", b"example"];
    let sp = Splitter::new(b"bytes example", s);
    let re: Vec<&[u8]> = vec![b"bytes", b" ", b"example"];
    assert_eq!(sp.collect::<Vec<_>>(), re);
}

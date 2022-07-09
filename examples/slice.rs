
use splitter::Splitter;

fn main() {
    let sp = Splitter::new(&[1, 2, 3, 3, 4], [[2], [4]]);
    let re: Vec<&[usize]> = vec![&[1], &[2], &[3, 3], &[4]];
    assert_eq!(sp.collect::<Vec<_>>(), re);
}

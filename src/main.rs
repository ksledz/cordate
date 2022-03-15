mod index;
use index::Index;
fn main() {
    let index = Index::new(b"ACGACGTACACGGTA", 3);
    println!("the result is {:?}",  index.get_interval(b"ACG", 3));
}

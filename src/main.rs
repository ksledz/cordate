mod index;
use index::Index;
use index::LookupTable;
use index::BitVector;
fn main() {
    let index = Index::<LookupTable>::new(b"ACGACGTACACGGTAACG", 3);
    println!("the LUT result is {:?}",  index.get_interval(b"ACG", 3));
    let index2 = Index::<BitVector>::new(b"ACGACGTACACGGTAACG", 3);
    println!("the bitvec result is {:?}",  index2.get_interval(b"ACG", 3));
}

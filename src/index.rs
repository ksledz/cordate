use fid_rs::Fid;
// TODO bitstream representation of text
use std::cmp::min;

fn rounded(a: usize, b: usize) -> usize {
    if a % b == 0 {
        return a / b;
    }
    ((a - (a % b)) / b) + 1
}

fn lex_num(a: &[u8]) -> usize {
    let mut res = 0;
    for c in a {
        res *= 4;
        res += match c {
            b'A' => 0,
            b'C' => 1,
            b'G' => 2,
            b'T' => 3,
            _ => unreachable!(),
        };
    }
    res
}

fn lex_next(a: &[u8]) -> Vec<u8>{
    let mut b = a.to_vec();
    for c in b.iter_mut().rev() {
        let (nc, cont) = match *c {
            b'A' => (b'C', false),
            b'C' => (b'G', false),
            b'G' => (b'T', false),
            b'T' => (b'A', true),
            _ => unreachable!(),
        };
        *c = nc;
        if !cont {
            break;
        }
    }
    b
}

pub struct LookupTable {
    table: Vec<usize>
}

pub struct BitVector {
    vector: fid_rs::Fid
}

pub trait PointerArray {
    fn _get_interval(&self, a: &[u8], length: usize) -> (usize, usize);
    fn new(text_rep: &[u8], perm_array: &[usize], length: usize) -> Self;
}

// pointer_array can be either FID or LookupTable
pub struct Index<B: PointerArray> {
    text_rep: Vec<u8>,
    perm_array: Vec<usize>,
    pointer_array: B,
}

impl Index<LookupTable> {
    pub fn new(text: &[u8], subword_len: usize) -> Self {
        let mut perm : Vec<usize> = (0..(rounded(text.len(),subword_len))).collect();
        perm.sort_by_key(|a| &text[a*subword_len..min((a+1)*subword_len, text.len())] );
        return Index{
            text_rep : text.to_vec(),
            pointer_array: LookupTable::new(text, &perm, subword_len),
            perm_array: perm,
        };
    }
    // TODO edge cases ?
    pub fn get_interval(&self, a: &[u8], length: usize) -> &[usize]{
        let (i0, i1) = self.pointer_array._get_interval(a, length);
        &self.perm_array[i0..i1]
    }
}

impl PointerArray for LookupTable {

    fn new(text_rep: &[u8], perm_array:&[usize], length: usize) -> Self {
       let mut lut = Vec::new();
       let mut l_item = b"A".repeat(length);
       for (i, item) in perm_array.iter().enumerate() {
           while text_rep[item*length..(item+1)*length] >= l_item[..] {
               lut.push(i);
               l_item = lex_next(&l_item);           
           }
       }
       LookupTable {
           table: lut
       }
    }

    fn _get_interval(&self, a: &[u8], length:usize) -> (usize, usize) {
        let mut a_padded : Vec<u8>= a.to_vec();
        a_padded.append(&mut b"A".repeat(length - a.len()));
        let a_padded_index = lex_num(&a_padded);
        let mut b_padded :Vec<u8>= lex_next(a);
        b_padded.append(&mut b"A".repeat(length - a.len()));
        let b_padded_index = lex_num(&b_padded);
        return (self.table[a_padded_index], self.table[b_padded_index]);
    }
}


impl Index<BitVector> {
    pub fn new(text: &[u8], subword_len: usize) -> Self {
        let mut perm : Vec<usize> = (0..(rounded(text.len(),subword_len))).collect();
        perm.sort_by_key(|a| &text[a*subword_len..min((a+1)*subword_len, text.len())] );
        return Index{
            text_rep : text.to_vec(),
            pointer_array: BitVector::new(text, &perm, subword_len),
            perm_array: perm,
        };
    }
    // TODO edge cases ?
    pub fn get_interval(&self, a: &[u8], length: usize) -> &[usize]{
        let (i0, i1) = self.pointer_array._get_interval(a, length);
        return &self.perm_array[i0..i1];
    }
}

impl PointerArray for BitVector {

    fn new(text_rep: &[u8], perm_array:&[usize], length: usize) -> Self {
       let mut string = String::new();
       let mut l_item = b"A".repeat(length);
       for(i, item) in perm_array.iter().enumerate() {
           while text_rep[item*length..(item+1)*length] > l_item[..] {
               string.push('1');
               l_item = lex_next(&l_item);           
           }
           if text_rep[item*length..(item+1)*length] == l_item[..] {
               string.push('0');
           }
       }
       return BitVector {
           vector: Fid::from(string.as_str()),
       };
    }
    // the interval of the permutation array corresponding to w starts at position rank0(select1(i0)) + 1 and ends at position rank0(select1(i1)).
    fn _get_interval(&self, a: &[u8], length:usize) -> (usize, usize) {
        let mut a_padded : Vec<u8>= a.to_vec();
        a_padded.append(&mut b"A".repeat(length - a.len()));
        let a_padded_index = lex_num(&a_padded) as u64;
        let mut b_padded :Vec<u8>= lex_next(a);
        b_padded.append(&mut b"A".repeat(length - a.len()));
        let b_padded_index = lex_num(&b_padded) as u64;
        let a_select = self.vector.select(a_padded_index).unwrap(); //option
        let b_select = self.vector.select(b_padded_index).unwrap();
        let a_res = self.vector.rank0(a_select) as usize;
        let b_res = self.vector.rank0(b_select) as usize;
        (a_res, b_res)
    }
}

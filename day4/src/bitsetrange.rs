use bit_set::BitSet;

pub trait SetRange {
    fn set_range(&mut self, from: usize, to: usize);
}

impl SetRange for BitSet {
    fn set_range(&mut self, from: usize, to: usize) {
        for i in from..to+1 {
            self.insert(i);
        }
    }
}


use std::clone::Clone;

// Sedgewick, p.271, 273
// Cormen, p.31

/// Merge sort (Top down).
///
/// Time complexity:
///
///     * best:     Ω(n log(n))
///     * avg:      Θ(n log(n))
///     * worst:    O(n log(n))
///
/// Space complexity:
///
///     * O(1)
pub fn sort<T: Ord + Clone>(input: &mut Vec<T>) {
    if input.len() == 0 { return; }
    let mut ms = MergeSort{tmp: input.to_vec()};
    let size = input.len() - 1;
    ms.merge_sort(input, 0, size);
}

struct MergeSort<T> {
    tmp: Vec<T>
}

impl<T: Ord + Clone> MergeSort<T> {
    fn merge_sort(&mut self, input: &mut Vec<T>, from: usize, to: usize) {
        if to <= from { return; }
        let middle = from + (to - from) / 2;
        self.merge_sort(input, from, middle);
        self.merge_sort(input, middle + 1, to);
        self.merge(input, from, middle, to);
    }

    fn merge(&mut self, input: &mut Vec<T>, from: usize, middle: usize, to: usize) {
        let mut i = from;
        let mut g = middle + 1;

        for k in from..(to + 1) {
            self.tmp[k] = input[k].clone();
        }

        for k in from..(to + 1) {
            if i > middle                       {input[k] = self.tmp[g].clone(); g = g + 1;}
            else if g > to                      {input[k] = self.tmp[i].clone(); i = i + 1;}
            else if self.tmp[g] < self.tmp[i]   {input[k] = self.tmp[g].clone(); g = g + 1;}
                else                            {input[k] = self.tmp[i].clone(); i = i + 1;}
        }
    }
}


add_common_tests!();

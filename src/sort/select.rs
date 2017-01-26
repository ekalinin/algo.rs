// Sedgewick, p.249
// Cormen, p.29
pub fn sort<T: Ord>(input: &mut Vec<T>) {
    let size = input.len();
    for i in 0..size {
        let mut min = i;
        for j in (i + 1)..size {
            if input[j] < input[min] { min = j; }
        }
        input.swap(i, min);
    }
}

add_common_tests!();
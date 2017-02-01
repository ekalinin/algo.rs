
// Cormen, p.40

/// Bubble sort.
///
/// Time complexity:
///
///     * best:     Ω(n)
///     * avg:      Θ(n^2)
///     * worst:    O(n^2)
///
/// Space complexity:
///
///     * O(1)
pub fn sort<T: Ord>(input: &mut Vec<T>) {
    loop {
        let mut swapped = false;
        for right in 1..input.len() {
            let left = right - 1;
            if input[left] > input[right] {
                input.swap(left, right);
                swapped = true;
            }
        }
        if !swapped { break; }
    }
}

add_common_tests!();
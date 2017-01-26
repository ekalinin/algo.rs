
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut tests = super::super::get_test_vecs();
        for t in tests.iter_mut() {
            let mut test_copy = t.clone();
            let test_slice = t.as_mut();
            println!("+ Unsorted: {:?}", test_slice);
            sort(test_slice);
            println!("-   Sorted: {:?}", test_slice);
            assert!(super::super::is_sorted(test_slice));
            test_copy.sort();
            assert!(super::super::is_eq(test_slice, test_copy.as_mut()));
        }
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;
    use self::test::Bencher;

    use super::*;

    fn bench_helper(b: &mut Bencher, size: i32) {
        let mut vec: Vec<_> = (0..size).rev().collect();
        b.iter(|| sort(vec.as_mut()));
    }

    #[bench]
    fn bench_worst_case_10(b: &mut Bencher) {
        bench_helper(b, 10);
    }

    #[bench]
    fn bench_worst_case_100(b: &mut Bencher) {
        bench_helper(b, 100);
    }

    #[bench]
    fn bench_worst_case_1000(b: &mut Bencher) {
        bench_helper(b, 1000);
    }

    #[bench]
    fn bench_worst_case_10000(b: &mut Bencher) {
        bench_helper(b, 10000);
    }
}

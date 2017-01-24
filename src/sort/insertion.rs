use std::fmt;

pub fn sort<T: Ord + fmt::Debug>(input: &mut Vec<T>) {
    for i in 1..input.len() {
        'back: for j in (1..(i+1)).rev() {  // as right border is not included, do: i+1
            if input[j] < input[j-1] {
                input.swap(j, j-1);
            } else {
                break 'back;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort(){
        let mut tests = super::super::get_test_vecs();
        for t in tests.iter_mut(){
            let test_slice = t.as_mut();
            println!("= Unsorted: {:?}", test_slice);
            sort(test_slice);
            println!("-  Sorted:   {:?}", test_slice);
            assert!(super::super::is_sorted(test_slice));
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

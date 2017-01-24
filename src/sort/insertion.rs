use std::fmt;

pub fn sort<T: Ord + fmt::Debug>(input: &mut Vec<T>) {
    for i in 1..input.len() {
        'back: for j in (1..(i+1)).rev() {
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
//    use test::Bencher;

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

//    #[bench]
//    fn bench_insertion_sort(b: &mut Bencher) {
//        b.iter(|| );
//    }
}

//    for test_vec in tests.iter_mut(){
//        let test_slice = test_vec.as_mut();
//        println("");
//        println!("> Unsorted: {:?}", test_slice);
//        sort(test_slice);
//        println!("  Sorted:   {:?}", test_slice);
//        assert!(super::is_sorted(test_slice));
//    }

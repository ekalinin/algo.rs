use std::fmt;

// Sedgewick edition (~30% faster than Cormen's one)
// p.251
pub fn sort<T: Ord + fmt::Debug>(input: &mut Vec<T>) {
    for i in 1..input.len() {
        // as right border is not included, do: i+1
        'back: for j in (1..(i + 1)).rev() {
            if input[j - 1] > input[j] {
                input.swap(j - 1, j);
            } else {
                break 'back;
            }
        }
    }
}

//use std::clone;
//// Cormen edition
//// p.18
//pub fn sort<T: Ord + fmt::Debug + clone::Clone>(input: &mut Vec<T>) {
//    for j in 1..input.len() {
//        let key = input[j].clone();
//        let mut i = j - 1;
//        // dirty hack: as "i" has usize type, it can't be < 0
//        // but we need to be able to update a first element of the vector
//        let mut insert_first = false;
//        'back: while input[i] > key {
//            input[i + 1] = input[i].clone();
//            if i == 0 {
//                insert_first = true;
//                break 'back;
//            } else {
//                i = i - 1;
//            }
//        }
//        input[if insert_first {0} else {i + 1}] = key;
//    }
//}


add_common_tests!();

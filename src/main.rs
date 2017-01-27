extern crate algo;

fn main() {
    let mut test = vec!(0,3,7,6);
    println!("Origin: {:?}", test);
    algo::sort::mergesort(test.as_mut());
    println!("Sorted: {:?}", test);
}
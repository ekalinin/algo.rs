extern crate algo;

fn main() {
    let mut test = vec!(19,19,21,21,22,25,19,14,23,25,14,10,8,4,28,12,2,33);
    println!("Origin: {:?}", test);
    algo::sort::mergesort(test.as_mut());
    println!("Sorted: {:?}", test);
}
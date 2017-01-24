extern crate algo;

fn main() {
    let mut test = vec!(8,5,2,6,9,3);
    println!("Origin: {:?}", test);
    algo::sort::insertion::sort(test.as_mut());
    println!("Sorted: {:?}", test);
}
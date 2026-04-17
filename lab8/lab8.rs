use std::cmp::Ord;

fn min<T: Ord>(a: T, b: T) -> T {
    if a <= b {
        a
    } else {
        b
    }
}

fn main() {
    println!("{}", min(3, 5));          // 3
    println!("{}", min(10, -1));        // -1
    println!("{}", min('b', 'a'));      // 'a'
}
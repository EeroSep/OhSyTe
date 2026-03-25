use std::hash::{DefaultHasher, Hash, Hasher};

fn main() {
    let student_number = std::env::args().nth(1).expect("no student number given");
    let mut hasher = DefaultHasher::new();
    student_number.hash(&mut hasher);
    println!("The hash for the student number is {:x}", hasher.finish());
}
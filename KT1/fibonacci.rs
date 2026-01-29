fn print_fib_num(n: u32) {
    let mut last2: u128 = 0;
    let mut last1: u128 = 1;
    let mut current: u128;
    for i in 0..n {
        if i <= 1 {
            println!("{}",i);
        } else {
            current = last1 + last2;
            println!("{}",current);
            last2 = last1;
            last1 = current;
        }
    }
}

fn main() {
    let amount = 10;
    print_fib_num(amount);
}

fn main() {
    let a = 10;
    println!("Hello, {}", a);

    for i in 1..10 {
        println!("Fib of {} is {}\n", i, fib(i));
    }

}

fn fib(num: u64) -> u64 {
    if num <= 2 {
        return 1;
    }

    else {
        return fib(num - 1) + fib(num - 2);
    }
}
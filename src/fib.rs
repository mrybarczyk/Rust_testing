fn fib(n: &i64, c: i64, s1: i64, s2:i64) -> i64 {
    if *n == c {
        s1
    } else {
        fib(&n, c+1, s2, s1+s2)
        // 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144...
    }
}

fn main() {
    let n = 3;
    let res = fib(&n, 1, 1, 1);
    println!("fibonacci for {} is {}", n, res);
}
fn main() {
    println!("生成第 n 个斐波那契数");
    let n: u64 = get_number();
    let fibonacci: u64 = fibonacci(n);
    println!("第 {} 个斐波那契数为：{}", n, fibonacci);
}

fn get_number() -> u64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("");
    return input.trim().parse().expect("");
}

fn fibonacci(n: u64) -> u64 {
    if n == 1 || n == 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

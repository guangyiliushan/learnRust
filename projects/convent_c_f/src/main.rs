fn main() {
    println!("相互转换摄氏与华氏温度");

    println!("请输入摄氏温度：");
    let celsius = get_temperature();
    println!("摄氏温度为：{}°C", celsius);
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("华氏温度为：{}°F", fahrenheit);

    println!("请输入华氏温度：");
    let fahrenheit = get_temperature();
    println!("华氏温度为：{}°F", fahrenheit);
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("摄氏温度为：{}°C", celsius);
}

fn get_temperature() -> f64 {
    let mut trytime = 0;
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("读取失败");
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("输入错误，请重新输入"),
        }
        input.clear();
        trytime += 1;
        if trytime > 3 {
            panic!("尝试次数过多");
        }
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}


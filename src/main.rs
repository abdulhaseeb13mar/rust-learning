use std::io::stdin;

fn main() {
    let mut f_temp = String::new();
    println!("Enter a temperature in Fahrenheit: ");
    stdin().read_line(&mut f_temp).expect("failed to read line");
    let f_temp: f32 = match f_temp.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid input"),
    };
    let f_temp = fahrenheit_to_celsius(f_temp);
    println!("{} degrees celsius", f_temp);
}

fn fahrenheit_to_celsius(temp: f32) -> f32 {
    let celsius = (temp - 32.0) * 5.0 / 9.0;
    celsius
}

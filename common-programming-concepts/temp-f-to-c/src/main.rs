use std::io;


fn main() {
    loop {
        println!("Enter a temp in Fahrenheit");
        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Filed to read the temperature");

        let temp: f32 = match temp.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        println!("Your temp in celcius is {}", temp_f_to_c(temp));
        break;
    }
    
}

fn temp_f_to_c(temp: f32) -> f32 {
    (temp - 32.0) * (5.0/9.0)
}
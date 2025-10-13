fn main() {
    println!("This is a simple F to C and C to F Temperature Converter:");
    println!("If you want to close the program, enter 'exit' or 'close':");
    println!("Enter your temperature here (example: 37 c):");

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("close") {
            println!("You have exited the program.");
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 2 {
            println!("Invalid input. Please enter like: 37 c");
            continue;
        }

        let value_res = parts[0].parse::<f64>();
        let unit = parts[1];

        let value = match value_res {
            Ok(v) => v,
            Err(_) => {
                println!("Invalid number. Please enter like: 34 f");
                continue;
            }
        };

        match unit.to_uppercase().as_str() {
            "C" => {
                let f = c_to_f(value);
                println!("{:.2}°C = {:.2}°F", value, f);
            }
            "F" => {
                let c = f_to_c(value);
                println!("{:.2}°F = {:.2}°C", value, c);
            }
            _ => {
                println!("Invalid unit. Must be 'c' or 'f'.");
                continue;
            }
        }
    }
}

fn c_to_f(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

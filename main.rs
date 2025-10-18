// Expense Tracker cli app in Rust
use std::fs::{File, OpenOptions} ;
use std::io::{self, BufReader, BufRead, Write} ;

#[derive(Debug)]
struct Expense {
    amount: f64,
    category: String,
    description: String,
}

impl Expense {
    fn new(amount: f64, category: String, description: String) -> Self {
        Expense {
            amount,
            category,
            description,
        }
    }
    fn to_csv_line(&self) -> String {
        format!("{}, {}, {}\n", self.amount, self.category, self.description)
    }
    fn from_csv_line(line: &str) -> Result<Self, String> {
        let parts: Vec<&str> = line.split(',').collect() ;
        if parts.len() != 3 {
            return Err("Invalid CSV format".to_string()) ;
        }
        let amount = parts[0].parse::<f64>().map_err(|_| "Invalid amount")?;

        Ok(Expense::new(
            amount,
            parts[1].to_string(),
            parts[2].to_string(),
        ))
    }
}




fn main() {
    println!("--------Expense Tracker__________") ;

    loop {
        println!("\n1. Add Expense : ");
        println!("2. View All Expense : ") ;
        println!("3. View Summary : ") ;
        println!("4. Exit : ");
        println!("\nChoose option : ") ;
        io::stdout().flush().unwrap() ;

        let mut choice = String::new() ;
        io::stdin().read_line(&mut choice).unwrap() ;

        match choice.trim() {
            "1" => add_expense() ,
            "2" => view_expense(),
            "3" => view_summary() ,
            "4" => {
                println!("Goodbye!") ;
                break ;
            }
            _ => println!("Invalid option!") ,
        }
    }
}

fn add_expense() {
    println!("Enter amount : ") ;
    io::stdout().flush().unwrap() ;
    let mut amount_str = String::new() ;
    io::stdin().read_line(&mut amount_str).unwrap() ;
    let amount: f64 = match amount_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid amount!") ;
            return ;
        }
    };
    println!("Enter category  (Food/Transport/Shopping/Other) : ") ;
    io::stdout().flush().unwrap() ;
    let mut category = String::new() ;
    io::stdin().read_line(&mut category).unwrap() ;

    println!("Enter description : ");
    io::stdout().flush().unwrap();
    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();

    let expense = Expense::new(
        amount,
        category.trim().to_string(),
        description.trim().to_string(),
    );
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("expense.csv")
        .unwrap();
    file.write_all(expense.to_csv_line().as_bytes()).unwrap() ;
    println!("Expense added successfully!") ;
}

fn view_expense() {
    let file = match File::open("expense.csv") {
        Ok(f) => f,
        Err(_) => {
            println!("No expense yet!") ;
            return ;
        }
    };
    let reader = BufReader::new(file) ;
    println!("\n---All Expense---") ;

    for line in reader.lines() {
        if let Ok(line) = line {
            if let Ok(expense) = Expense::from_csv_line(&line) {
                println!(
                    "Amount: Tk {:.2} | Category: {} | Description: {}",
                    expense.amount, expense.category, expense.description
                );
            }
        }
    }
}

fn view_summary() {
    let file = match File::open("expense.csv") {
        Ok(f) => f,
        Err(_) => {
            println!("No expense yet!");
            return;
        }
    };

    let reader = BufReader::new(file);
    let mut total = 0.0 ;
    let mut category_totals: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new() ;

    for line in reader.lines() {
        if let Ok(line) = line {
            if let Ok(expense) = Expense::from_csv_line(&line) {
                total += expense.amount;
                *category_totals.entry(expense.category.clone()).or_insert(0.0) += expense.amount ;

            }
        }
    }
    println!("\n----Summary----") ;
    println!("Total Expense: Tk {:.2}", total) ;
    println!("\nBy Category") ;
    for (category, amount) in category_totals {
        println!("{}: Tk{:.2}", category, amount) ;
    }

}

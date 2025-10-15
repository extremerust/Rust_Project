use std::io;

fn main() {
    println!("Simple To-Do List App") ;
    println!("-----------------------") ;

    let mut todos: Vec<String> = Vec::new() ;
    loop {
        println!("\nWhat are you doing ? ") ;
        println!(" Enter : 1 -> to add task ") ;
        println!("Enter : 2 -> to see all tasks") ;
        println!("Enter : 3 -> to delete task") ;
        println!("Enter : 4 -> to close the program") ;


        let mut choice = String::new() ;
        io::stdin().read_line(&mut choice).unwrap() ;
        match choice.trim() {
            "1" => add_task(&mut todos) ,
            "2" => view_task(&todos) ,
            "3" => remove_task(&mut todos) ,
            "4" => {
                println!("The program is closeing...-> Bye") ;
                break ;
            }
            _ => println!("Invalid input! Plz Enter 1-4 : ") ,
        }
    }
}

fn add_task(todos: &mut Vec<String>) {
    println!("Enter New task here : ") ;
    let mut task = String::new() ;
    io::stdin().read_line(&mut task).unwrap() ;
    let task =  task.trim() ;

    if task.is_empty() {
        println!("The task is doesn't be empty!") ;
       return ;
    }
    todos.push(task.to_string()) ;
    println!("{} add done!", task) ;
}

fn view_task(todos: &Vec<String>) {
    if todos.is_empty() {
        println!("There is no task!");
        return;
    }
    println!("\nAll Task...") ;
    for (i, task) in todos.iter().enumerate() {
        println!("{}. {}", i + 1, task) ;
    }
}

fn remove_task(todos: &mut Vec<String>) {
    if todos.is_empty() {
        println!("Sorry the task is already empty") ;
        return ;
    }
    view_task(todos) ;
    println!("Which task you want to delete? Enter number for that: ") ;

    let mut input = String::new() ;
    io::stdin().read_line(&mut input).unwrap() ;

    let index: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter valid number : ") ;
            return ;
        }
    };

    if index == 0 || index > todos.len() {
        println!("Invalid number!") ;
        return ;
    }

    let removed = todos.remove(index - 1) ;
    println!("{} remove done!", removed) ;
}

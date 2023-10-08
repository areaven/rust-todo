use dirs::home_dir;
use std::{env, fs, io::stdin, path};

fn main() -> std::io::Result<()> {
    // parse command
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Le erraste che, metele un comando válido.");
        return Ok(());
    }
    let command = &args[1].to_lowercase();

    // read todo file
    let mut path = home_dir().expect("Error: Unable to fetch home path.");
    path.push("todo");
    if !path.exists() {
        println!("File doesn't exist! Creating...");
        fs::File::create(&path)?;
    }

    // execute the command
    match command.as_str() {
        "add" => add_task(&path),
        "list" => display_tasks(&path),
        "del" => remove_task(&path),
        _ => {
            eprintln!("Le erraste che, metele un comando válido.");
            return Ok(());
        }
    }
    Ok(())
}

fn display_tasks(path: &path::Path) {
    let todo_string = fs::read_to_string(path).expect("Something went wrong!");
    for (n, task) in todo_string.lines().enumerate() {
        println!("{}. {}", n + 1, task)
    }
}
fn add_task(path: &path::Path) {
    let mut task = String::new();
    println!("What's the task?: ");
    stdin().read_line(&mut task).expect("Something went wrong.");
    let mut todo_string = fs::read_to_string(path).expect("Something went wrong!");

    todo_string.push_str(&task);
    fs::write(path, todo_string).expect("Something went wrong.");
}
fn remove_task(path: &path::Path) {
    let mut task_number = String::new();
    println!("Number of task to remove: ");
    stdin()
        .read_line(&mut task_number)
        .expect("Something went wrong.");
    let task_number: usize = task_number.trim().parse().expect("Something went wrong.");
    let todo_string = fs::read_to_string(path).expect("Something went wrong!");
    let mut todo_string: Vec<&str> = todo_string.lines().collect();
    todo_string.remove(task_number - 1);
    let todo_string = todo_string.join("\n");
    fs::write(path, todo_string).expect("Something went wrong.");
}

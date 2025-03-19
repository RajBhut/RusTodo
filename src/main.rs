use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
struct TodoItem {
    task: String,
    done: bool,
}

const FILE_PATH: &str = "todos.json";

#[derive(Parser)]
#[command(name = "Rust Todo App", about = "A simple command-line todo list")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { task: String },
    List,
    Complete { index: usize },
    Delete { index: usize },
}

fn load_todos() -> Result<Vec<TodoItem>, Box<dyn Error>> {
    let mut file = match File::open(FILE_PATH) {
        Ok(f) => f,
        Err(_) => return Ok(vec![]),
    };

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let todos: Vec<TodoItem> = serde_json::from_str(&content)?;
    Ok(todos)
}

fn save_todos(todos: &Vec<TodoItem>) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(&todos)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH)?;

    file.write_all(json.as_bytes())?;
    Ok(())
}

fn add_todo(task: String) -> Result<(), Box<dyn Error>> {
    let mut todos = load_todos()?;

    todos.push(TodoItem { task, done: false });

    save_todos(&todos)?;
    println!("✅ Task added!");
    Ok(())
}

fn show_todos() -> Result<(), Box<dyn Error>> {
    let todos = load_todos()?;

    if todos.is_empty() {
        println!("📂 No todos found!");
    } else {
        println!("📋 Your Todos:");
        for (i, todo) in todos.iter().enumerate() {
            println!(
                "{}: [{}] {}",
                i + 1,
                if todo.done { "✔" } else { " " },
                todo.task
            );
        }
    }
    Ok(())
}

fn complete_todo(index: usize) -> Result<(), Box<dyn Error>> {
    let mut todos = load_todos()?;

    if index == 0 || index > todos.len() {
        println!("⚠️ Invalid task number!");
        return Ok(());
    }

    todos[index - 1].done = true;
    save_todos(&todos)?;
    println!("✅ Task {} marked as completed!", index);
    Ok(())
}

fn delete_todo(index: usize) -> Result<(), Box<dyn Error>> {
    let mut todos = load_todos()?;

    if index == 0 || index > todos.len() {
        println!("⚠️ Invalid task number!");
        return Ok(());
    }

    todos.remove(index - 1);
    save_todos(&todos)?;
    println!("🗑️ Task {} deleted!", index);
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { task } => add_todo(task).unwrap_or_else(|e| eprintln!("Error: {}", e)),
        Commands::List => show_todos().unwrap_or_else(|e| eprintln!("Error: {}", e)),
        Commands::Complete { index } => {
            complete_todo(index).unwrap_or_else(|e| eprintln!("Error: {}", e))
        }
        Commands::Delete { index } => {
            delete_todo(index).unwrap_or_else(|e| eprintln!("Error: {}", e))
        }
    }
}

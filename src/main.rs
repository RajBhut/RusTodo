struct TODO {
    task: String,
    completed: bool,
}
impl TODO {
    fn new(task: String) -> Self {
        Self {
            task,
            completed: false,
        }
    }
}
fn main() {
    let mut todo_list: Vec<TODO> = Vec::new();
    todo_list.push(TODO::new("Learning Rust".to_string()));
    todo_list.push(TODO::new("Trying to be Good".to_string()));

    for item in todo_list {
        print!("Task: {} , completed : {} \n", item.task, item.completed)
    }
}

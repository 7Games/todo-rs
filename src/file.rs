use crate::todoitem::*;
use std::fs::File;
use std::io::Write;

pub fn save_list(todo_list: &Vec<TodoItem>) {
    let path = "todo.list";
    let mut output = File::create(path).unwrap();
    
    for i in 0..todo_list.len() {
        let line = (if todo_list[i].ticked == true {"true"} else {"false"}).to_owned() + &"\t" + &todo_list[i].name + &"\n";
        write!(output, "{}", line).unwrap();
    }
}

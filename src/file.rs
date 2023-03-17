use crate::todoitem::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn save_list(todo_list: &Vec<TodoItem>) {
    let path = "todo.list";
    let mut output = File::create(path).unwrap();
    
    for i in 0..todo_list.len() {
        let line = (if todo_list[i].ticked == true {"true"} else {"false"}).to_owned() + &"\t" + &todo_list[i].name + &"\n";
        write!(output, "{}", line).unwrap();
    }
}

pub fn load_list(todo_list: &mut Vec<TodoItem>) {
    let path = "todo.list";
    let contents = BufReader::new(File::open(path).expect("Unable to open file"));

    for line in contents.lines() {
        let line_un: String = line.unwrap();
        let parts = line_un.split("\t");
        let collection = &parts.collect::<Vec<&str>>();
        let mut _ticked: bool = false;
        let mut _name: String = String::new();

        if collection[0] == "true" {
            _ticked = true;
        } else if collection[0] == "false" {
            _ticked = false;
        } else {
            panic!("Item not valid");
        }

        _name = collection[1].to_string();

        let item: TodoItem = TodoItem { name: _name.to_string(), ticked: _ticked };

        todo_list.push(item);
    }
}

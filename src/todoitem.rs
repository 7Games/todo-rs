pub struct TodoItem {
    pub name: String,
    pub ticked: bool,
}

pub fn print_list(todo_list: &Vec<TodoItem>) {
    for i in 0..todo_list.len() {
        println!(" {} [{}] {}", (i + 1), (if todo_list[i].ticked == true {"x"} else {" "}), todo_list[i].name);
    }
}

pub fn add_item(name: String, ticked: bool, todo_list: &mut Vec<TodoItem>) {
    let item: TodoItem = TodoItem { name: name.to_string(), ticked: ticked };
    todo_list.push(item);
}

pub fn delete_item(index: usize, todo_list: &mut Vec<TodoItem>) {
    todo_list.remove(index);
}

pub fn tick_item(index: usize, todo_list: &mut Vec<TodoItem>) {
    todo_list[index].ticked = !todo_list[index].ticked;
}

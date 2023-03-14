mod todoitem;

use crate::todoitem::*;

fn menu_ui() {
    let mut todo_list: Vec<TodoItem> = Vec::new();

    add_item("Buy eggs".to_string(), false, &mut todo_list);
    add_item("Call mum".to_string(), false, &mut todo_list);
    add_item("Instal Gentoo".to_string(), true, &mut todo_list);

    delete_item(2, &mut todo_list);
    tick_item(1, &mut todo_list);

    if todo_list.len() == 0 {
        println!("(none)");
    } else {
        print_list(todo_list);
    }
}

fn main() {
    menu_ui();
}

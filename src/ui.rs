use crate::todoitem::*;
use crate::file::*;
use std::io::Write;
use std::{thread, time};

fn clear_terminal() {
    print!("{}[2J", 27 as char);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn add_item_ui(todo_list: &mut Vec<TodoItem>) {
    let mut item_name: String = String::new();
    let mut option: String = String::new();

    clear_terminal();

    print!("Enter item name: ");

    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut item_name).unwrap();
    item_name = item_name.replace('\n', "");

    loop {
        clear_terminal();
        print!("Do you want to add “{}” to the list?\n\x1b[4mY\x1b[0mes/\x1b[4mN\x1b[0mo  ", item_name);

        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut option).unwrap();
        option = option.replace('\n', "");
        option = option.to_lowercase();

        if option == "y" || option == "yes" {
            add_item(item_name, false, todo_list);
            break;
        } else if option == "n" || option == "no" {
            break;
        }
    }
}

fn select_item_ui(todo_list: &mut Vec<TodoItem>, thing: &str) {
    if todo_list.len() == 0 {
        println!("List is empty");
        thread::sleep(time::Duration::from_millis(2000));
        return;
    }

    loop {
        let mut option: String = String::new();
        let mut _index: usize = 0;

        print!("Select an index (enter '0' to stop): ");

        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut option).unwrap();
        option = option.replace('\n', "");
        _index = match option.parse::<usize>() {
            Ok(i) => i,
            Err(e) => {
                println!("Input is not a valid number; {}", e);
                thread::sleep(time::Duration::from_millis(1000));
                continue;
            },
        };

        if _index == 0 {
            break;
        }

        if _index < 1 || _index > todo_list.len() {
            println!("Not a valid index");
            thread::sleep(time::Duration::from_millis(3000));
            continue;
        }

        if thing == "delete" {
            loop {
                option = String::new();

                clear_terminal();
                print!("Are you sure you want to remove “{}” from the list?\n\x1b[4mY\x1b[0mes/\x1b[4mN\x1b[0mo  ", todo_list[_index - 1].name);
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut option).unwrap();
                option = option.replace('\n', "");
                option = option.to_lowercase();
    
                if option == "yes" || option == "y" {
                    break;
                } else if option == "no" || option == "n" {
                    return;
                } else {
                    continue;
                }
            }

            delete_item(_index - 1, todo_list);
        } else if thing == "tick" {
            tick_item(_index - 1, todo_list);
        }
        break;
    }
}

pub fn menu_ui() {
    let mut todo_list: Vec<TodoItem> = Vec::new();

    loop {
        clear_terminal();

        let mut input = String::new();

        if todo_list.len() == 0 {
            println!("(none)");
        } else {
            print_list(&todo_list);
        }

        println!("===========================================");
        println!("\x1b[4mC\x1b[0mreate | \x1b[4mD\x1b[0melete | \x1b[4mT\x1b[0mick | \x1b[4mS\x1b[0mave | \x1b[4mL\x1b[0moad | \x1b[4mQ\x1b[0muit");
        print!("> ");

        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.replace('\n', "");
        input = input.to_lowercase();

        if input == "create" || input == "c" {
            add_item_ui(&mut todo_list);
        } else if input == "delete" || input == "d" {
            select_item_ui(&mut todo_list, "delete");
        } else if input == "tick" || input == "t" {
            select_item_ui(&mut todo_list, "tick");
        } else if input == "save" || input == "s" {
            input = String::new();

            clear_terminal();
            print!("Are you sure you want to save this list to disk?\nAny list saved before will be deleted!\n\x1b[4mY\x1b[0mes/\x1b[4mN\x1b[0mo  ");

            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut input).unwrap();
            input = input.replace('\n', "");
            input = input.to_lowercase();

            if input == "y" || input == "yes" {
                save_list(&todo_list);
            }
        } else if input == "load" || input == "l" {
            input = String::new();

            clear_terminal();
            print!("Are you sure you want to load a list from disk?\nThis will clear your current list!\n\x1b[4mY\x1b[0mes/\x1b[4mN\x1b[0mo  ");

            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut input).unwrap();
            input = input.replace('\n', "");
            input = input.to_lowercase();

            if input == "y" || input == "yes" {
                load_list(&mut todo_list);
            }
        } else if input == "quit" || input == "q" {
            input = String::new();

            clear_terminal();
            print!("Do you want to quit? Your list will not be saved!\n\x1b[4mY\x1b[0mes/\x1b[4mN\x1b[0mo  ");

            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut input).unwrap();
            input = input.replace('\n', "");
            input = input.to_lowercase();

            if input == "y" || input == "yes" {
                clear_terminal();
                break;
            }
        }
    }
}

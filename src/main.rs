mod todo;

use std::io;

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action.");
    if action == "add" {
        add_task();
    } else if action == "complete" {
        mark_as_complete();
    } else if action == "show" {
        show_todo();
    } else if action == "remove" {
        remove_task();
    } else if action == "edit" {
        edit_task_name();
    }
}

fn edit_task_name() {
    let mut todo = get_todo();
    print!("To Edit ");
    let item = read_task_name();
    print!("Edit to ");
    let new_name = read_task_name();
    match todo.edit(&item, new_name) {
        None => println!("'{}' is not present in the list", item),
        Some(_) => match todo.save() {
            Ok(_) => println!("item edited"),
            Err(why) => println!("An error occurred: {}", why),
        },
    }
}

fn remove_task() {
    let mut todo = get_todo();
    let item = read_task_name();
    match todo.remove(&item) {
        None => println!("'{}' is not present in the list", item),
        Some(_) => match todo.save() {
            Ok(_) => println!("item removed"),
            Err(why) => println!("An error occurred: {}", why),
        },
    }
}

fn show_todo() {
    let todo = get_todo();
    todo.show();
}

fn mark_as_complete() {
    let mut todo = get_todo();
    let item = read_task_name();
    match todo.complete(&item) {
        None => println!("'{}' is not present in the list", item),
        Some(_) => match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        },
    }
}

fn add_task() {
    let mut todo = get_todo();
    let item = read_task_name();
    todo.insert(item);
    match todo.save() {
        Ok(_) => println!("todo saved!"),
        Err(why) => println!("An error occurred: {}", why),
    }
}

fn get_todo() -> todo::Todo {
    todo::Todo::new().expect("Initialization of db failed!")
}

fn read_task_name() -> String {
    let mut item = String::new();
    let stdin = io::stdin();
    println!("Task name:");
    stdin.read_line(&mut item).expect("Error on reading the input.");
    return item.trim().to_string();
}

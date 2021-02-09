mod todo;

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    let mut todo = todo::Todo::new().expect("Initialisation of db failed");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Saved todo "),
            Err(error) => println!("An error occured: {}", error),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Todo updated and saved"),
                Err(error) => println!("An error occured: {}", error),
            },
        }
    } else if action == "delete" {
        match todo.delete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Todo has been deleted"),
                Err(error) => println!("An error occured {}", error),
            },
        }
    } else if action == "display" {
        todo.display();
    }
}

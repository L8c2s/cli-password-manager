mod menu;
mod db_service;

fn main() {
    test_database();

    println!("{}", "-".repeat(50));
    println!();
    // this is never called by me, but the program runs anyway.
    // this might mean that I need to keep it here.

    let options = ["New Entry", "Manage Accounts"];
    let choice = menu::simple_menu("Home", &options, "Quit");
    println!();

    if choice < options.len() {
        println!("Option chosen: {}", options[choice]);
    } else {
        println!("Exiting");
    }
}

fn test_database() {
}

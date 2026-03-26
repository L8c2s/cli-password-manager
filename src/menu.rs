use std::io::Write;
use std::io;


fn show_title(title: &str) {
    let title_char = "=";

    let mut title_len = title.chars().count();
    let title_len_min = 50;

    if title_len < title_len_min {
        title_len = title_len_min;
    } else if title_len > title_len_min {
        title_len = title_len + 2;
    }

    println!("{}", title_char.repeat(title_len));
    println!("{}", format!("{:^width$}", title, width=title_len));
    println!("{}", title_char.repeat(title_len));
}

fn show_menu(options: &[&str], _exit: &str) -> usize {
    let options_len = options.len();

    for (index, option) in options.iter().enumerate() {
        println!("[{}] {}", index, option);
    }
    println!("[{}] {}", options_len, _exit);

    // get user input
    print!("\n>>> ");
    io::stdout().flush().expect("Flush failed!");

    let mut str_choice = String::new();

    io::stdin()
        .read_line(&mut str_choice)
        .expect("Failed to read line.");

    let choice = str_choice.trim().parse::<usize>().unwrap_or(usize::MAX);

    if choice > options_len {
        println!("Please, select a valid option.\n");
    }

    choice
}

pub fn simple_menu(title: &str, options: &[&str], _exit: &str) -> usize {
    show_title(title);
    show_menu(options, _exit)
}

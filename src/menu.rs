fn _show_title(title: &str) {
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

fn _show_menu() {
    // receives an array of options to display on the menu 
    // the menu shows all items listed starting from 0 and 
    // going all the way to the number of the length of the 
    // options array
    // also, the function adds a new option that is the 
    // cancel/exit option. the number of this option is 
    // the same as the length of the array.
}

pub fn simple_menu(title: &str) {
    _show_title(title);
}

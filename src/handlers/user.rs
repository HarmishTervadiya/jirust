use crate::utils::get_input;

pub fn show_user_menu() {
    loop {
        println!("\n+-----------------------------+");
        println!("|          USER MENU         |");
        println!("+-----------------------------+");
        println!("| [1] Create a new user      |");
        println!("| [2] Update user details    |");
        println!("| [3] Delete user            |");
        println!("| [4] Get user details       |");
        println!("| [5] Back                   |");
        println!("+-----------------------------+");
        print!("Enter choice (1-5): ");

        let input = get_input();
        match input.as_str() {
            "5"=> break,
            _ => println!("Invalid option. Please enter a number from 1 to 4."),
        }
    }
}
use crate::models::user::User;
use crate::state::AppState;
use crate::utils::get_input;

fn create_user(state: &mut AppState) {
    println!("\nCreate User");
    print!("Enter name: ");
    let name = get_input();
    if name.is_empty() {
        println!("Name cannot be empty.");
        return;
    }

    print!("Enter email: ");
    let email = get_input();
    if email.is_empty() {
        println!("Email cannot be empty.");
        return;
    }

    let email_exists = state
        .users
        .values()
        .any(|user| user.email.eq_ignore_ascii_case(&email));

    if email_exists {
        println!("A user with this email already exists.");
        return;
    }

    state.next_id += 1;
    let new_id = state.next_id;

    state.users.insert(
        new_id,
        User {
            id: new_id,
            name,
            email,
        },
    );

    println!("User created successfully with id: {}", new_id);
}

fn get_users(state: &mut AppState) {
    println!("\nGet User");

    println!("\nAll Users");
    if state.users.is_empty() {
        println!("No users found.");
        return;
    }

    println!("+----+----------------+----------------------+");
    println!("| ID | Name           | Email                |");
    println!("+----+----------------+----------------------+");

    for user in state.users.values() {
        println!("ID: {}", user.id);
        println!("Name: {}", user.name);
        println!("Email: {}", user.email);
        println!("---------------------------");
    }

    println!("+----+----------------+----------------------+");
}

pub fn show_user_menu(state: &mut AppState) {
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
            "1" => create_user(state),
            "4" => get_users(state),
            "5" => break,
            _ => println!("Invalid option. Please enter a number from 1 to 5."),
        }
    }
}



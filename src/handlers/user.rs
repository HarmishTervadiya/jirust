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

fn read_user_id() -> Option<u32> {
    print!("Enter id: ");
    let raw_id = get_input();

    match raw_id.parse::<u32>() {
        Ok(id) => Some(id),
        Err(_) => {
            println!("Invalid id. Please enter a valid number.");
            None
        }
    }
}

fn print_user_details(user: &User) {
    println!("ID: {}", user.id);
    println!("Name: {}", user.name);
    println!("Email: {}", user.email);
    println!("---------------------------");
}

fn get_users(state: &mut AppState) {
    println!("\nGet User");

    println!("\nAll Users");
    if state.users.is_empty() {
        println!("No users found.");
        return;
    }

    for user in state.users.values() {
        print_user_details(user);
    }
}

fn get_user_by_id(state: &mut AppState) {
    println!("\nGet User By Id");
    let Some(id) = read_user_id() else {
        return;
    };

    println!("\nUser with id: {}", id);
    if state.users.is_empty() {
        println!("No users found.");
        return;
    }

    let user = state.users.get(&id);
    match user {
        Some(user) => {
            print_user_details(user);
        }
        None => {
            println!("No user found.");
        }
    }
}

pub fn show_user_menu(state: &mut AppState) {
    loop {
        println!("\n+------------------------------+");
        println!("|           USER MENU            |");
        println!("+--------------------------------+");
        println!("| [1] Create a new user          |");
        println!("| [2] Update user details        |");
        println!("| [3] Delete user                |");
        println!("| [4] Get all user details       |");
        println!("| [5] Get user by id             |");
        println!("| [6] Back                       |");
        println!("+--------------------------------+");
        print!("Enter choice (1-6): ");

        let input = get_input();
        match input.as_str() {
            "1" => create_user(state),
            "2" => update_user_by_id(state),
            "3" => delete_user_by_id(state),
            "4" => get_users(state),
            "5" => get_user_by_id(state),
            "6" => break,
            _ => println!("Invalid option. Please enter a number from 1 to 6."),
        }
    }
}

pub fn update_user_by_id(state: &mut AppState) {
    println!("\nUpdate User By Id");
    let Some(id) = read_user_id() else {
        return;
    };

    match state.users.get_mut(&id) {
        Some(user) => {
            println!("\nCurrent user details:");
            print_user_details(user);

            print!("Enter name to update or leave blank: ");
            let name = get_input();
            print!("Enter email to update or leave blank: ");
            let email = get_input();

            if !name.is_empty() {
                user.name = name;
            }

            if !email.is_empty() {
                user.email = email;
            }

            println!("User updated successfully.");
        }
        None => {
            println!("User does not exist.");
        }
    }
}

pub fn delete_user_by_id(state: &mut AppState) {
    println!("\nDelete User By Id");

    if state.users.is_empty() {
        println!("No users found.");
        return;
    }

    get_users(state);

    let Some(id) = read_user_id() else {
        return;
    };

    match state.users.remove(&id) {
        Some(user) => {
            println!("User deleted successfully.");
            println!("\nDeleted user details:");
            print_user_details(&user);
        }
        None => {
            println!("User does not exist.");
        }
    }
}

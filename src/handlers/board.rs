use crate::handlers::organization::get_all_org;
use crate::models::board::Board;
use crate::state::AppState;
use crate::utils::{get_input, read_id};

pub fn show_board_menu(state: &mut AppState) {
    loop {
        println!("\n+------------------------------+");
        println!("|          BOARD MENU          |");
        println!("+------------------------------+");
        println!("| [1] Create board             |");
        println!("| [2] Update board             |");
        println!("| [3] Delete board             |");
        println!("| [4] Get all boards           |");
        println!("| [5] Get board by id          |");
        println!("| [6] Back                     |");
        println!("+------------------------------+");
        print!("Enter choice (1-6): ");

        let input = get_input();
        match input.as_str() {
            "1" => create_board(state),
            "2" => update_board_by_id(state),
            "3" => delete_board_by_id(state),
            "4" => get_all_boards(state),
            "5" => get_board_by_id(state),
            "6" => break,
            _ => println!("Invalid option. Please enter a number from 1 to 6."),
        }
    }
}

fn create_board(state: &mut AppState) {
    println!("\nCreate Board");
    print!("Enter board name: ");
    let board_name = get_input();

    if board_name.is_empty() {
        println!("Board name cannot be empty.");
        return;
    }

    println!("Select org from any of these");
    get_all_org(state);

    let Some(org_id) = read_id() else {
        return;
    };

    if state.organizations.get(&org_id).is_none() {
        println!("Organization does not exist");
        return;
    }

    let board_exists = state.boards.values().any(|board| {
        board.name.eq_ignore_ascii_case(&board_name) && board.organization_id == org_id
    });

    if board_exists {
        println!("Board already exists in this organization.");
        return;
    }

    state.next_id += 1;
    let board_id = state.next_id;

    state.boards.insert(board_id, {
        Board {
            id: board_id,
            name: board_name,
            organization_id: org_id,
        }
    });

    println!("Board created successfully with id: {}", board_id);
}

fn update_board_by_id(state: &mut AppState) {
    println!("\nUpdate Board By Id");
    let Some(id) = read_id() else {
        return;
    };

    let Some(current_board) = state.boards.get(&id) else {
        println!("Board does not exist.");
        return;
    };

    println!("\nCurrent board details:");
    show_board_details(current_board);

    print!("Enter board name to update or leave blank: ");
    let board_name = get_input();

    println!("Select organization from below or leave blank");
    get_all_org(state);
    print!("Enter organization id to update or leave blank: ");
    let org_id_input = get_input();

    let parsed_org_id = if org_id_input.is_empty() {
        None
    } else {
        let Ok(org_id) = org_id_input.parse::<u32>() else {
            println!("Invalid id. Please enter a valid number.");
            return;
        };

        if state.organizations.get(&org_id).is_none() {
            println!("Organization does not exist.");
            return;
        }

        Some(org_id)
    };

    let Some(board) = state.boards.get_mut(&id) else {
        println!("Board does not exist.");
        return;
    };

    if !board_name.is_empty() {
        board.name = board_name;
    }

    if let Some(org_id) = parsed_org_id {
        board.organization_id = org_id;
    }

    println!("Board updated successfully.");
}

fn delete_board_by_id(state: &mut AppState) {
    println!("\nDelete Board By Id");

    if state.boards.is_empty() {
        println!("No boards found.");
        return;
    }

    get_all_boards(state);

    let Some(id) = read_id() else {
        return;
    };

    let Some(board) = state.boards.remove(&id) else {
        println!("Board does not exist.");
        return;
    };

    println!("Board deleted successfully.");
    println!("\nDeleted board details:");
    show_board_details(&board);
}

pub fn get_all_boards(state: &AppState) {
    println!("\nAll Boards");

    if state.boards.is_empty() {
        println!("No boards found.");
        return;
    }

    for board in state.boards.values() {
        show_board_details(board);
    }
}

fn get_board_by_id(state: &AppState) {
    println!("\nGet Board By Id");
    let Some(id) = read_id() else {
        return;
    };

    if state.boards.is_empty() {
        println!("No boards found.");
        return;
    }

    println!("\nBoard with id: {}", id);
    let Some(board) = state.boards.get(&id) else {
        println!("Board does not exist.");
        return;
    };

    show_board_details(board);
}

pub fn show_board_details(board: &Board) {
    println!("ID: {}", board.id);
    println!("Name: {}", board.name);
    println!("Organization ID: {}", board.organization_id);
    println!("---------------------------");
}

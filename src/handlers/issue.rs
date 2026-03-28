use crate::handlers::board::get_all_boards;
use crate::handlers::user::get_users;
use crate::models::issue::{Issue, IssueStatus};
use crate::state::AppState;
use crate::utils::{get_input, read_id};

pub fn show_issue_menu(state: &mut AppState) {
    loop {
        println!("\n+------------------------------+");
        println!("|      ISSUE MENU              |");
        println!("+------------------------------+");
        println!("| [1] Create issue             |");
        println!("| [2] Update issue             |");
        println!("| [3] Delete issue             |");
        println!("| [4] Get all issues           |");
        println!("| [5] Get issue by id          |");
        println!("| [6] Back                     |");
        println!("+------------------------------+");
        print!("Enter choice (1-6): ");
        let input = get_input();

        match input.as_str() {
            "1" => create_issue(state),
            "2" => update_issue_by_id(state),
            "3" => delete_issue_by_id(state),
            "4" => get_all_issue(state),
            "5" => get_issue_by_id(state),
            "6" => break,
            _ => println!("Invalid option. Please enter a number from 1 to 6."),
        }
    }
}

fn create_issue(state: &mut AppState) {
    println!("\nCreate issue");
    print!("Enter issue title: ");
    let issue_title = get_input();
    if issue_title.is_empty() {
        println!("issue title cannot be empty.");
        return;
    }

    let issue_exists = state
        .issues
        .values()
        .any(|issue| issue.title.eq_ignore_ascii_case(&issue_title));

    if issue_exists {
        println!("issue with this title already exists.");
        return;
    }

    print!("Enter issue description: ");
    let issue_description = get_input();
    if issue_description.is_empty() {
        println!("issue description cannot be empty.");
        return;
    }

    print!("Enter issue status: ");
    let issue_status = get_input().to_lowercase();
    if issue_status.is_empty()
        || (issue_status != "open" && issue_status != "inprogress" && issue_status != "done")
    {
        println!("Invalid issue status.");
        return;
    }

    let status = match issue_status.parse::<IssueStatus>() {
        Ok(s) => s,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    println!("Select assignee from any of these");
    get_users(state);

    let Some(user_id) = read_id() else {
        return;
    };

    if state.users.get(&user_id).is_none() {
        println!("User does not exist");
        return;
    }

    println!("Select board from any of these");
    get_all_boards(state);

    let Some(board_id) = read_id() else {
        return;
    };

    if state.boards.get(&board_id).is_none() {
        println!("board does not exist");
        return;
    }

    state.next_id += 1;
    let issue_id = state.next_id;

    state.issues.insert(
        issue_id,
        Issue {
            id: issue_id,
            title: issue_title,
            description: issue_description,
            status: status,
            board_id: board_id,
            assignee_id: user_id,
        },
    );

    println!("issue created successfully with id: {}", issue_id);
}

fn get_issue_by_id(state: &AppState) {
    println!("\nGet issue By Id");
    let Some(id) = read_id() else {
        return;
    };

    if state.issues.is_empty() {
        println!("No issue found.");
        return;
    }

    println!("\nissue with id: {}", id);

    let Some(issue) = state.issues.get(&id) else {
        println!("issue not found");
        return;
    };

    print_issue_details(issue);
}

fn update_issue_by_id(state: &mut AppState) {
    println!("\nUpdate issue By Id");

    let Some(id) = read_id() else {
        return;
    };

    if state.issues.get(&id).is_none() {
        println!("issue does not exist.");
        return;
    }

    println!("Select assignee from any of these to update or leave blank:");
    get_users(state);

    let user_input = read_id();

    if let Some(user_id) = user_input {
        if state.users.get(&user_id).is_none() {
            println!("User does not exist");
            return;
        }
    }

    println!("Select board from any of these");
    get_all_boards(state);

    let board_input = read_id();

    if let Some(board_id) = board_input {
        if state.boards.get(&board_id).is_none() {
            println!("board does not exist");
            return;
        }
    }

    print!("Enter issue title to update or leave blank: ");
    let issue_title = get_input();

    print!("Enter issue description to update or leave blank: ");
    let issue_description = get_input();

    print!("Enter issue status to update or leave blank: ");
    let issue_status = get_input().to_lowercase();

    let issue = state.issues.get_mut(&id).unwrap();

    println!("\nCurrent issue details:");
    print_issue_details(issue);

    if !issue_title.is_empty() {
        issue.title = issue_title;
    }

    if !issue_description.is_empty() {
        issue.description = issue_description;
    }

    if !issue_status.is_empty() {
        match issue_status.as_str() {
            "open" => issue.status = IssueStatus::Open,
            "inprogress" => issue.status = IssueStatus::InProgress,
            "done" => issue.status = IssueStatus::Done,
            _ => println!("Invalid status, keeping old value."),
        }
    }

    if let Some(user_id) = user_input {
        issue.assignee_id = user_id;
    }

    if let Some(board_id) = board_input {
        issue.board_id = board_id;
    }

    println!("issue updated successfully.");
}

fn delete_issue_by_id(state: &mut AppState) {
    println!("\nDelete issue By Id");

    if state.issues.is_empty() {
        println!("No issue found.");
        return;
    }

    get_all_issue(state);

    let Some(id) = read_id() else {
        return;
    };

    let Some(issue) = state.issues.remove(&id) else {
        println!("issue does not exist.");
        return;
    };

    println!("issue deleted successfully.");
    println!("\nDeleted issue details:");
    print_issue_details(&issue);
}

pub fn get_all_issue(state: &AppState) {
    println!("\nAll issues");
    if state.issues.is_empty() {
        println!("No issue found.");
        return;
    }

    for issue in state.issues.values() {
        print_issue_details(issue);
    }
}

pub fn print_issue_details(issue: &Issue) {
    println!("ID: {}", issue.id);
    println!("Title: {}", issue.title);
    println!("Description: {}", issue.description);
    println!("Status: {:?}", issue.status);
    println!("Board: {}", issue.board_id);
    println!("Assignee: {}", issue.assignee_id);
    println!("---------------------------");
}

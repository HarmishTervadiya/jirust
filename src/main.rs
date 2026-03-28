mod utils;
mod handlers {
    pub mod board;
    pub mod organization;
    pub mod user;
}
mod state;
mod models {
    pub mod board;
    pub mod issue;
    pub mod organization;
    pub mod user;
}

use std::collections::HashMap;

use crate::handlers::board::show_board_menu;
use crate::handlers::organization::show_organization_menu;
use crate::handlers::user::show_user_menu;
use crate::state::AppState;
use crate::utils::{get_input, show_main_menu};

fn main() {
    let mut state: AppState = AppState {
        users: HashMap::new(),
        organizations: HashMap::new(),
        boards: HashMap::new(),
        issues: HashMap::new(),
        next_id: 0,
    };
    loop {
        show_main_menu();
        match get_input().as_str() {
            "1" => show_organization_menu(&mut state),
            "2" => show_user_menu(&mut state),
            "3" => show_board_menu(&mut state),
            "4" => println!("issue option"),
            "q" => break,
            _ => println!("Invalid option"),
        }
    }
}

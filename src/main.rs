mod utils;
mod handlers {
    pub mod user;
}
mod state;
mod models {
    pub mod user;
    pub mod organization;
    pub mod issue;
    pub mod board;
}

use std::collections::HashMap;

use crate::utils::{show_main_menu, get_input};
use crate::handlers::user::{show_user_menu};
use crate::state::AppState;

fn main() {
    let mut state:AppState= AppState{
        users: HashMap::new(),
        organizations: HashMap::new(),
        boards: HashMap::new(),
        issues: HashMap::new(),
        next_id: 0,
    } ;
    loop {
        show_main_menu();
        // match get_input().as_str() {
        //     "1" => org_handler::menu(&mut state),
        //     "2" => user_handler::menu(&mut state),
        //     "3" => board_handler::menu(&mut state),
        //     "4" => issue_handler::menu(&mut state),
        //     "q" => break,
        //     _   => println!("Invalid option"),
        // }
        match get_input().as_str() {
            "1" => println!("org option"),
            "2" => show_user_menu(&mut state),
            "3" => println!("board option"),
            "4" => println!("issue option"),
            "q" => break,
            _   => println!("Invalid option"),
        }
    }
}

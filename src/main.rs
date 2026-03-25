mod utils;

use crate::utils::{show_main_menu, get_input};

fn main() {
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
            "2" => println!("user option"),
            "3" => println!("board option"),
            "4" => println!("issue option"),
            "q" => break,
            _   => println!("Invalid option"),
        }
    }
}

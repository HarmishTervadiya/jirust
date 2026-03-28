use std::io::{self, Write};

// * get input 
pub fn get_input() -> String {
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// * menu 
pub fn show_main_menu() {
    println!("\n╔══════════════════════════════╗");
    println!("║       🦀 JIRUST v1.0.0       ║");
    println!("╠══════════════════════════════╣");
    println!("║  Manage:                     ║");
    println!("║   [1] Organizations          ║");
    println!("║   [2] Users                  ║");
    println!("║   [3] Boards                 ║");
    println!("║   [4] Issues                 ║");
    println!("╠══════════════════════════════╣");
    println!("║   [q] Quit                   ║");
    println!("╚══════════════════════════════╝");
    print!("\n  Enter choice: ");
}

pub fn read_id() -> Option<u32> {
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

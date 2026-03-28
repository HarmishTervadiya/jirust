use crate::models::organization::Organization;
use crate::state::AppState;
use crate::utils::{get_input, read_id};

pub fn show_organization_menu(state: &mut AppState) {
    loop {
        println!("\n+------------------------------+");
        println!("|      ORGANIZATION MENU       |");
        println!("+------------------------------+");
        println!("| [1] Create organization      |");
        println!("| [2] Update organization      |");
        println!("| [3] Delete organization      |");
        println!("| [4] Get all organizations    |");
        println!("| [5] Get organization by id   |");
        println!("| [6] Back                     |");
        println!("+------------------------------+");
        print!("Enter choice (1-6): ");
        let input = get_input();

        match input.as_str() {
            "1" => create_organization(state),
            "2" => update_org_by_id(state),
            "3" => delete_org_by_id(state),
            "4" => get_all_org(state),
            "5" => get_org_by_id(state),
            "6" => break,
            _ => println!("Invalid option. Please enter a number from 1 to 6."),
        }
    }
}

fn create_organization(state: &mut AppState) {
    println!("\nCreate Organization");
    print!("Enter org name: ");
    let org_name = get_input();
    if org_name.is_empty() {
        println!("Organization name cannot be empty.");
        return;
    }

    let org_exists = state
        .organizations
        .values()
        .any(|org| org.name.eq_ignore_ascii_case(&org_name));

    if org_exists {
        println!("Organization with this name already exists.");
        return;
    }

    state.next_id += 1;
    let org_id = state.next_id;

    state.organizations.insert(
        org_id,
        Organization {
            id: org_id,
            name: org_name,
        },
    );

    println!("Organization created successfully with id: {}", org_id);
}

fn get_org_by_id(state: &AppState) {
    println!("\nGet Organization By Id");
    let Some(id) = read_id() else {
        return;
    };

    if state.organizations.is_empty() {
        println!("No organization found.");
        return;
    }

    println!("\nOrganization with id: {}", id);

    let Some(org) = state.organizations.get(&id) else {
        println!("Organization not found");
        return;
    };

    print_org_details(org);
}

fn update_org_by_id(state: &mut AppState) {
    println!("\nUpdate Organization By Id");
    let Some(id) = read_id() else {
        return;
    };

    let Some(organization) = state.organizations.get_mut(&id) else {
        println!("Organization does not exist.");
        return;
    };

    println!("\nCurrent organization details:");
    print_org_details(organization);

    print!("Enter organization name to update or leave blank: ");
    let org_name = get_input();

    if !org_name.is_empty() {
        organization.name = org_name;
    }

    println!("Organization updated successfully.");
}

fn delete_org_by_id(state: &mut AppState) {
    println!("\nDelete Organization By Id");

    if state.organizations.is_empty() {
        println!("No organization found.");
        return;
    }

    get_all_org(state);

    let Some(id) = read_id() else {
        return;
    };

    let Some(organization) = state.organizations.remove(&id) else {
        println!("Organization does not exist.");
        return;
    };

    println!("Organization deleted successfully.");
    println!("\nDeleted organization details:");
    print_org_details(&organization);
}

pub fn get_all_org(state: &AppState) {
    println!("\nAll Organizations");
    if state.organizations.is_empty() {
        println!("No organization found.");
        return;
    }

    for organization in state.organizations.values() {
        print_org_details(organization);
    }
}

pub fn print_org_details(organization: &Organization) {
    println!("ID: {}", organization.id);
    println!("Name: {}", organization.name);
    println!("---------------------------");
}

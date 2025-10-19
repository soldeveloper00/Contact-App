use std::io;

#[derive(Clone)]
struct Contact {
    name: String,
    phone: String,
}

struct ContactApp {
    contacts: Vec<Contact>,
}

impl ContactApp {
    fn new() -> Self {
        ContactApp {
            contacts: Vec::new(),
        }
    }

    // Insert contact
    fn insert_contact(&mut self, name: String, phone: String) {
        self.contacts.push(Contact { name, phone });
        // sort ascending by name
        self.contacts.sort_by(|a, b| a.name.cmp(&b.name));
        println!("Inserted successfully!");
    }

    // Delete contact
    fn delete_contact(&mut self, name: &str) {
        if let Some(index) = self.contacts.iter().position(|c| c.name == name) {
            self.contacts.remove(index);
            println!("Deleted successfully!");
        } else {
            println!("Contact not found!");
        }
    }

    // Show all contacts
    fn show_contacts(&self) {
        if self.contacts.is_empty() {
            println!("No contacts available.");
        } else {
            println!("\n--- Contact List ---");
            for c in &self.contacts {
                println!("Name: {} | Phone: {}", c.name, c.phone);
            }
        }
    }

    // Edit contact
    fn edit_contact(&mut self, name: &str, new_name: String, new_phone: String) {
        if let Some(contact) = self.contacts.iter_mut().find(|c| c.name == name) {
            contact.name = new_name;
            contact.phone = new_phone;
            // Sort after editing
            self.contacts.sort_by(|a, b| a.name.cmp(&b.name));
            println!("Contact updated!");
        } else {
            println!("Contact not found!");
        }
    }

    // Search contact
    fn search_contact(&self, name: &str) {
        if let Some(contact) = self.contacts.iter().find(|c| c.name == name) {
            println!(
                "Found -> Name: {} | Phone: {}",
                contact.name, contact.phone
            );
        } else {
            println!("Contact not found!");
        }
    }
}

// Helper function to read input
fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut app = ContactApp::new();
    print!("Build by soldeveloper00\n");

    loop {
        println!("\n--- Contact App ---");
        println!("1. Insert");
        println!("2. Delete");
        println!("3. Show");
        println!("4. Edit");
        println!("5. Search");
        println!("6. Exit");

        let choice = read_input("Enter your choice (1-6):");

        match choice.as_str() {
            "1" => {
                let name = read_input("Enter name:");
                let phone = read_input("Enter phone:");
                app.insert_contact(name, phone);
            }
            "2" => {
                let name = read_input("Enter name to delete:");
                app.delete_contact(&name);
            }
            "3" => app.show_contacts(),
            "4" => {
                let name = read_input("Enter name to edit:");
                let new_name = read_input("Enter new name:");
                let new_phone = read_input("Enter new phone:");
                app.edit_contact(&name, new_name, new_phone);
            }
            "5" => {
                let name = read_input("Enter name to search:");
                app.search_contact(&name);
            }
            "6" => {
                println!("Exiting program...");
                break;
            }
            _ => println!("Invalid choice! Try again."),
        }
    }
}
// End of code
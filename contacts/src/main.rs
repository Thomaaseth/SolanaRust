use std::fs::File;
use std::io::{Write, Error};

struct Contact {
    name: String,
    number: String
}

// add new contact
fn add_contact(contacts: &mut Vec<Contact>) {
    println!("Add contact name and phone number");
    // ask name, phone
    println!("Enter contact name:");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Error");

    println!("Enter contact number:");
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).expect("Error");

    // add contact to vector
    let new_contact = Contact {
        name: name,
        number: number
    };
    contacts.push(new_contact)
}

// show contacts
fn show_all_contacts(contacts: &mut Vec<Contact>) {

    contacts.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    for contact in contacts.iter() {
        println!("Name: {}, Number: {}", contact.name, contact.number)
    }
    println!("Total contacts: {}", contacts.len())
}

fn export_all_contacts(contacts: &[Contact]) {
    let mut file = File::create("contacts.csv").expect("Failed to create file");
    file.write_all(b"Name,Phone\n").expect("Failed to write header");

    for contact in contacts {
        let line = format!("{},{}\n", contact.name.trim(), contact.number.trim());
        file.write_all(line.as_bytes()).expect("Failed to write contact");
    }
    println!("Contacts exported successfully!");
}

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();

    loop {
        println!("\n=== MENU ===");
        println!("1. Ajouter un contact");
        println!("2. Afficher tous les contacts");
        println!("3. Exporter les contacts");

        let mut choix = String::new();
        std::io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        let choix = choix.trim();

        // matching selon le choix, appeler la bonne methode
        match choix {
            "1" => add_contact(&mut contacts),
            "2" => show_all_contacts(&mut contacts),
            "3" => export_all_contacts(&contacts),
            // "4" => quit(),
            _ => print!("Fail")
        } 
    }
}
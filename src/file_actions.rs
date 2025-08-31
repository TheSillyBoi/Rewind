use std::io::{BufReader, Write};
use crate::Reminder;
use std::fs::File;
use std::path::Path;
use std::env;
use xml::reader::{EventReader, XmlEvent};



pub fn read_reminders() -> Result<Vec<Reminder>, Box<dyn std::error::Error>> {
    let file = File::open(get_file_path())?;
    let parser = EventReader::new(BufReader::new(file));
    
    let mut reminders = Vec::new();
    let mut current_reminder = Reminder { name: String::new(), time: String::new() };
    let mut current_element = String::new();
    let mut inside_reminder = false;
    
    for event in parser {
        match event? {
            XmlEvent::StartElement { name, .. } => {
                let element_name = name.local_name;
                if element_name == "reminder" {
                    inside_reminder = true;
                    current_reminder = Reminder { name: String::new(), time: String::new() };
                }
                current_element = element_name;
            }
            XmlEvent::Characters(data) => {
                if inside_reminder && !data.trim().is_empty() {
                    match current_element.as_str() {
                        "name" => current_reminder.name = data.trim().to_string(),
                        "time" => current_reminder.time = data.trim().to_string(),
                        _ => {}
                    }
                }
            }
            XmlEvent::EndElement { name } => {
                if name.local_name == "reminder" {
                    reminders.push(current_reminder.clone());
                    inside_reminder = false;
                }
            }
            _ => {}
        }
    }
    
    Ok(reminders)
}

fn get_file_path() -> String {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    format!("{}/.cache/Rewinders.xml", home)
}

pub fn does_file_exist() {
    let file_path = get_file_path();
    if !Path::new(&file_path).exists() {
        println!("File doesn't exist at {}, will be created on first save", file_path);
    } else {
        println!("Found existing file at {}", file_path);
    }
}

pub fn write_reminders(reminders: &Vec<Reminder>) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(get_file_path())?;
    
    writeln!(file, "<reminders>")?;
    
    for reminder in reminders {
        writeln!(file, "  <reminder>")?;
        writeln!(file, "    <name>{}</name>", reminder.name)?;
        writeln!(file, "    <time>{}</time>", reminder.time)?;
        writeln!(file, "  </reminder>")?;
    }
    
    writeln!(file, "</reminders>")?;
    
    Ok(())
}

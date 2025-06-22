use gtk::glib::clone;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, WidgetExt, PopoverExt, EntryExt, EditableExt,FrameExt,};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};
use xml::reader::{EventReader, XmlEvent};
use std::fs::File;
use std::io::{BufReader, Write};
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};

struct AppModel {
    counter: i8,
    main_window: gtk::Window,
    reminders: Vec<Reminder>,
}

#[derive(Debug, Clone)]
struct Reminder {
    name: String,
    time: String,
}

fn read_reminders() -> Result<Vec<Reminder>, Box<dyn std::error::Error>> {
    let file = File::open("StoredData.xml")?;
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

fn write_reminders(reminders: &Vec<Reminder>) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("StoredData.xml")?;
    
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

#[derive(Debug)]
enum AppMsg {
    NewReminder,
    FinalizeReminder(String, String),  // (name, iso_date)
    Quit,
}

struct AppWidgets {
    menu_button: gtk::MenuButton,
    new_tracked: gtk::Button,
}

impl SimpleComponent for AppModel {
    type Input = AppMsg;
    type Output = ();
    type Init = u8;
    type Root = gtk::Window;
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        gtk::Window::builder()
            .title("Rewind")
            .default_width(700)
            .default_height(500)
            .build()
    }

    fn init(
        counter: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let existing_reminders = read_reminders().unwrap();

        let model = AppModel { 
            counter: counter.try_into().unwrap(), 
            main_window: window.clone(),
            reminders: existing_reminders, 
        };
        let header = gtk::HeaderBar::new();
        let menu_button = gtk::MenuButton::new();
        let new_tracked = gtk::Button::new();
        
        header.pack_end(&menu_button);
        header.pack_end(&new_tracked);
        window.set_titlebar(Some(&header));
        
        menu_button.set_icon_name("open-menu-symbolic");
        new_tracked.set_icon_name("list-add");

        let menu_dropdown = gtk::Popover::new();
        let popover_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .build();
        
        let exit_button = gtk::Button::with_label("Exit");
        popover_box.append(&exit_button);
        menu_dropdown.set_child(Some(&popover_box));
        menu_button.set_popover(Some(&menu_dropdown));

        // Main content area
        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .build();
        
        window.set_child(Some(&vbox));
        vbox.set_margin_all(5);

        for reminder in &model.reminders {
            let reminder_frame = gtk::Frame::new(Some(&reminder.name));
            let reminder_label = gtk::Label::new(Some(&format!("Due: {}", reminder.time)));

            // Parse ISO 8601 string back to DateTime
            let parsed_datetime = DateTime::parse_from_str(&reminder.time, "%Y-%m-%dT%H:%M:%S%z")
                .or_else(|_| {
                    // If no timezone, assume local
                    NaiveDateTime::parse_from_str(&reminder.time, "%Y-%m-%dT%H:%M:%S")
                        .map(|dt| Local.from_local_datetime(&dt).unwrap().into())
                });

            reminder_frame.set_child(Some(&reminder_label));
            vbox.append(&reminder_frame);
        }

        // Event handlers
        exit_button.connect_clicked(clone!(
            #[strong] sender,
            move |_| {
                sender.input(AppMsg::Quit);
            }
        ));

        new_tracked.connect_clicked(clone!(
            #[strong] sender,
            move |_| {
                sender.input(AppMsg::NewReminder);
            }
        ));

        let widgets = AppWidgets { menu_button, new_tracked };
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppMsg::Quit => {
                std::process::exit(0);
            }

            
            AppMsg::FinalizeReminder(text, iso_date) => {
                let new_reminder = Reminder {
                    name: text,
                    time: iso_date,  // Use the actual date from calendar
                };
                
                self.reminders.push(new_reminder);
                
                // Write all reminders back to XML file
                if let Err(e) = write_reminders(&self.reminders) {
                    println!("Error writing to XML: {}", e);
                } else {
                    println!("Successfully saved {} reminders to XML", self.reminders.len());
                }
            }
            
            AppMsg::NewReminder => {
                let reminder_window = gtk::Window::builder()
                    .title("Add new Reminder")
                    .default_width(400)
                    .default_height(300)
                    .build();
                
                let reminderbox = gtk::Box::builder()
                    .orientation(gtk::Orientation::Vertical)
                    .spacing(5)
                    .margin_start(45)
                    .margin_end(45)
                    .build();
                
                let calendar = gtk::Calendar::new();
                let reminder_name = gtk::Entry::new();
                reminder_name.set_placeholder_text(Some("What is the Reminder Called?"));
                
                let finalize = gtk::Button::new();
                finalize.set_icon_name("checkmark-symbolic");
                
                finalize.connect_clicked(clone!(
                    #[strong] sender,
                    #[strong] reminder_name,
                    #[strong] calendar,
                    move |_| {
                        let text = reminder_name.text().to_string();
                        
                        // Get date from calendar and convert to ISO string
                        let gtk_date = calendar.date();
                        let year = gtk_date.year();
                        let month = gtk_date.month() as u32;
                        let day = gtk_date.day_of_month() as u32;
                        
                        let naive_date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
                        let naive_time = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
                        let naive_datetime = NaiveDateTime::new(naive_date, naive_time);
                        let local_datetime: DateTime<Local> = Local.from_local_datetime(&naive_datetime).unwrap();
                        let iso_string = local_datetime.format("%Y-%m-%dT%H:%M:%S").to_string();
                        
                        sender.input(AppMsg::FinalizeReminder(text, iso_string));
                    }
                ));
                
                reminderbox.append(&reminder_name);
                reminderbox.append(&calendar);
                reminderbox.append(&finalize);
                reminder_window.set_child(Some(&reminderbox));
                reminder_window.set_transient_for(Some(&self.main_window));
                reminder_window.set_modal(true);
                reminder_window.present();
            }
        }
    }
}

fn main() {

    let app = RelmApp::new("relm4.test.simple_manual");
    app.run::<AppModel>(0);
}
use gtk::glib::clone;
use gtk::{ MessageType, DialogFlags, ButtonsType, ResponseType};
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, PopoverExt, EntryExt, EditableExt, FrameExt, WidgetExt, DialogExt};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, SimpleComponent};
use xml::reader::{EventReader, XmlEvent};
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use std::env;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use notify_rust::{Notification,Timeout,Hint};
use std::collections::HashSet;
use std::cell::RefCell;
use gtk::CssProvider;
use gtk::StyleContext;
use gtk::gdk::Display;

struct AppModel {
    main_window: gtk::Window, 
    reminders: Vec<Reminder>,
}

#[derive(Debug, Clone)]
struct Reminder {
    name: String,
    time: String,
}
fn read_reminders() -> Result<Vec<Reminder>, Box<dyn std::error::Error>> {
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
    format!("{}/Rewinders.xml", home)
}

fn does_file_exist() {
    let file_path = get_file_path();
    if !Path::new(&file_path).exists() {
        println!("File doesn't exist at {}, will be created on first save", file_path);
    } else {
        println!("Found existing file at {}", file_path);
    }
}

fn write_reminders(reminders: &Vec<Reminder>) -> Result<(), Box<dyn std::error::Error>> {
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

fn apply_css() {
    let provider = CssProvider::new();

    // Load CSS from main.css file - load_from_path returns () in GTK4
    provider.load_from_path("main.css");
    println!("CSS loaded from main.css");

    // Get the default display
    let display = Display::default().expect("Could not get default display");

    // Add provider with priority using the non-deprecated function
    gtk::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

#[derive(Debug)]
enum AppMsg {
    NewReminder,
    FinalizeReminder(String, String),
    LoadInitialData, 
    Quit,
    About,
}

struct AppWidgets {
    menu_button: gtk::MenuButton,
    new_tracked: gtk::Button,
    reminder_container: gtk::Box,
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
        _whydoineedthis: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let existing_reminders = read_reminders().unwrap_or_else(|_| Vec::new());

        let model = AppModel { 
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
        let about_button = gtk::Button::with_label("About");
        popover_box.append(&exit_button);
        popover_box.append(&about_button);
        menu_dropdown.set_child(Some(&popover_box));
        menu_button.set_popover(Some(&menu_dropdown));

        let scrolled_window = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Automatic)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .build();

        let reminder_container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .build();

        
        scrolled_window.set_child(Some(&reminder_container));
        window.set_child(Some(&scrolled_window));
        about_button.connect_clicked(clone!(
            #[strong] sender,
            move |_| {
                sender.input(AppMsg::About);
            }

        ));
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

        let widgets = AppWidgets { 
            menu_button, 
            new_tracked,
            reminder_container: reminder_container.clone(),
        };

        sender.input(AppMsg::LoadInitialData);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppMsg::LoadInitialData => {
            }

            AppMsg::Quit => {
                std::process::exit(0);
            }

            AppMsg::FinalizeReminder(text, iso_date) => {
                self.reminders.push(Reminder {
                    name: text,
                    time: iso_date,
                });
                
                if let Err(e) = write_reminders(&self.reminders) {
                    println!("Error writing to XML: {}", e);
                } else {
                    println!("Successfully saved {} reminders to XML", self.reminders.len());
                }
            }
            AppMsg::About => {
                let about_window = gtk::AboutDialog::new();
                about_window.set_program_name(Some("Rewind"));
                about_window.set_comments(Some("A simple reminder app in order to learn GUIs, Storing Data, and the concepts thereof"));
                about_window.set_authors(&["Adrian Tennies"]);
                about_window.set_website(Some("https://github.com/thesillyboi/Rewind"));
                about_window.set_copyright(Some("©2025 Adrian Tennies"));
                about_window.set_license(Some("LGPL-2.1 License"));
                
                match gtk::gdk::Texture::from_filename("Logo.png") {
                    Ok(texture) => {
                        about_window.set_logo(Some(&texture));
                    },
                    Err(e) => {
                        println!("Could not load logo.png: {}", e);
                        // Optionally set a default icon name instead
                        about_window.set_logo_icon_name(Some("application-x-executable"));
                    }
                }
                
                about_window.set_transient_for(Some(&self.main_window));
                about_window.present();
            }
            AppMsg::NewReminder => {
                let reminder_window = gtk::Dialog::builder()
                    .title("Add new Reminder")
                    .default_width(600)
                    .default_height(750)
                    .build();
                
                let reminderbox = gtk::Box::builder()
                    .orientation(gtk::Orientation::Vertical)
                    .spacing(5)
                    .margin_start(45)
                    .margin_end(45)
                    .build();

                let hour_adjustment = gtk::Adjustment::new(
                    12.0,  
                    0.0,   
                    23.0,  
                    1.0,   
                    1.0,   
                    0.0    
                );
                let reminder_hour = gtk::SpinButton::new(Some(&hour_adjustment), 1.0, 0);

                let minute_adjustment = gtk::Adjustment::new(
                    0.0,   
                    0.0,   
                    59.0,  
                    1.0,   
                    5.0,   
                    0.0    
                );
                let reminder_minute = gtk::SpinButton::new(Some(&minute_adjustment), 1.0, 0);

                
                let calendar = gtk::Calendar::new();
                let reminder_name = gtk::Entry::new();
                reminder_name.set_placeholder_text(Some("What is the Reminder Called?"));
                reminder_name.set_max_length(100);
                reminder_name.add_css_class("remindername");
                
                // Debug: Check if the CSS class was added

                let finalize = gtk::Button::new();
                finalize.set_icon_name("checkmark-symbolic");

                reminderbox.append(&reminder_name);
                reminderbox.append(&reminder_hour);
                reminderbox.append(&reminder_minute);
                reminderbox.append(&calendar);
                reminderbox.append(&finalize);
                reminder_window.set_child(Some(&reminderbox));
                reminder_window.set_transient_for(Some(&self.main_window));
                reminder_window.set_modal(true);
                reminder_window.present();
                
                finalize.connect_clicked(clone!(
                    #[strong] sender,
                    #[strong] reminder_name,
                    #[strong] calendar,
                    #[strong] reminder_window,
                    #[strong] reminder_hour,
                    #[strong] reminder_minute,
                    move |_| {
                        let text = reminder_name.text().to_string();
                        
                        let gtk_date = calendar.date();
                        let year = gtk_date.year();
                        let month = gtk_date.month() as u32;
                        let day = gtk_date.day_of_month() as u32;
                        let minute = reminder_minute.value_as_int() as u32;
                        let hour = reminder_hour.value_as_int() as u32;

                        let naive_date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
                        let naive_time = NaiveTime::from_hms_opt(hour, minute, 0).unwrap();
                        let naive_datetime = NaiveDateTime::new(naive_date, naive_time);
                        let local_datetime: DateTime<Local> = Local.from_local_datetime(&naive_datetime).unwrap();
                        let iso_string = local_datetime.format("%Y-%m-%dT%H:%M:%S").to_string();

                        // Validate that the reminder time is in the future
                        let current_local = Local::now();
                        if local_datetime <= current_local {
                            println!("Reminder time must be in the future!");
                            let reminder_ood = gtk::MessageDialog::new(
                                Some(&reminder_window), 
                                DialogFlags::MODAL, 
                                MessageType::Error,
                                ButtonsType::Ok, 
                                "The Reminder must be in the Future!"
                            );
                            reminder_ood.connect_response(move |dialog, response| {
                                match response {
                                    ResponseType::Ok => {
                                        dialog.close();
                                    }, 
                                    _ => {
                                        println!("Box closed");
                                        dialog.close();
                                    }
                                }
                            });
                            reminder_ood.present();
                            return;
                        }

                        println!("{}", iso_string);
                        sender.input(AppMsg::FinalizeReminder(text, iso_string));
                        reminder_window.close(); 
                    }
                ));
                
                
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        let mut child = widgets.reminder_container.first_child();
        while let Some(widget) = child {
            let next = widget.next_sibling();
            widgets.reminder_container.remove(&widget);
            child = next;
        }
        
        for reminder in &self.reminders {
            let reminder_frame = gtk::Frame::new(Some(&reminder.name));
            
            // Center the frame label
            reminder_frame.set_label_align(0.5);
            
            let naive_dt = NaiveDateTime::parse_from_str(&reminder.time, "%Y-%m-%dT%H:%M:%S")
                .expect("Failed to parse datetime");     
            let readable = naive_dt.format("%A, %B %e, %Y at %H:%M:%S").to_string();
            let reminder_label = gtk::Label::new(Some(&format!("Due: {}", readable)));
            reminder_frame.set_child(Some(&reminder_label));
            widgets.reminder_container.append(&reminder_frame);
        }
    }
}

fn main() {
    // Initialize GTK first
    gtk::init().expect("Failed to initialize GTK");
    
    // Apply CSS styling
    apply_css();
    
    does_file_exist();
   
    let notified_reminders: std::rc::Rc<RefCell<HashSet<String>>> = std::rc::Rc::new(RefCell::new(HashSet::new()));
    
    // Track past reminders to avoid notifying for them
    let past_reminders: std::rc::Rc<RefCell<HashSet<String>>> = std::rc::Rc::new(RefCell::new(HashSet::new()));
    
    // Try a second test notification after a short delay
    gtk::glib::timeout_add_seconds_local(2, || {
        println!("Sending secondary test notification");
        match Notification::new()
            .summary("Second Test")
            .body("Testing notifications from timeout callback...")
            .appname("Rewind")
            .timeout(Timeout::Milliseconds(5000))
            .hint(Hint::Urgency(notify_rust::Urgency::Critical))
            .show() {
            Ok(_) => println!("Secondary test notification sent successfully"),
            Err(e) => println!("Secondary test notification failed: {}", e),
        }
        gtk::glib::ControlFlow::Break  // Run only once
    });
    
    // Mark all existing past reminders on startup
    if let Ok(reminders) = read_reminders() {
        let current_naive = Local::now().naive_local();
        let mut past = past_reminders.borrow_mut();
        
        for reminder in reminders {
            if let Ok(reminder_time) = NaiveDateTime::parse_from_str(&reminder.time, "%Y-%m-%dT%H:%M:%S") {
                if reminder_time < current_naive {
                    // This is a past reminder, mark it
                    let reminder_id = format!("{}_{}", reminder.name, reminder.time);
                    past.insert(reminder_id);
                    println!("Marked past reminder: {}", reminder.name);
                }
            }
        }
    }
    
    gtk::glib::timeout_add_seconds_local(15, clone!(
        #[strong] notified_reminders,
        #[strong] past_reminders,
        move || {
            println!("Checking reminders...");
            
            if let Ok(reminders) = read_reminders() {
                let current_local = Local::now();
                let current_naive = current_local.naive_local();
                
                println!("Current time: {}", current_naive);
                println!("Found {} reminders", reminders.len());
                
                for reminder in reminders {
                    if let Ok(reminder_time) = NaiveDateTime::parse_from_str(&reminder.time, "%Y-%m-%dT%H:%M:%S") {
                        let time_diff = reminder_time.signed_duration_since(current_naive);
                        let reminder_id = format!("{}_{}", reminder.name, reminder.time);
                        
                        println!("Reminder '{}' time: {}, diff: {} seconds", 
                                 reminder.name, reminder_time, time_diff.num_seconds());
                        
                        // Check if this is a past reminder we've already seen
                        if past_reminders.borrow().contains(&reminder_id) {
                            println!("Skipping past reminder: {}", reminder.name);
                            continue;
                        }
                        
                        // Only notify for upcoming reminders or those due in the last 60 seconds
                        if time_diff.num_seconds() >= -60 && time_diff.num_seconds() <= 300 {
                            // Check if we've already notified for this reminder
                            let mut notified = notified_reminders.borrow_mut();
                            if !notified.contains(&reminder_id) {
                                println!("Sending notification for: {}", reminder.name);
                                
                                // Try with more specific notification settings
                                match Notification::new()
                                    .summary(&format!("Reminder: {}", reminder.name))
                                    .body(&format!("Your reminder '{}' is due now!", reminder.name))
                                    .icon("appointment-soon")
                                    .timeout(Timeout::Milliseconds(10000))
                                    .hint(Hint::Urgency(notify_rust::Urgency::Critical))
                                    .hint(Hint::Category("reminder".to_string()))
                                    .show() {
                                    Ok(_) => {
                                        println!("Notification sent for: {}", reminder.name);
                                        notified.insert(reminder_id.clone());
                                        
                                        // Also try native command as fallback
                                        let cmd = format!(
                                            "notify-send -u critical \"Reminder: {}\" \"Your reminder is due now!\"",
                                            reminder.name
                                        );
                                        match std::process::Command::new("sh")
                                            .arg("-c")
                                            .arg(&cmd)
                                            .status() {
                                            Ok(_) => println!("Sent fallback notification via command"),
                                            Err(e) => println!("Failed to send fallback: {}", e),
                                        }
                                    },
                                    Err(e) => println!("Failed to send notification: {}", e),
                                }
                            } else {
                                println!("Already notified for: {}", reminder.name);
                            }
                        } else if time_diff.num_seconds() < 0 {
                            // This reminder is in the past, mark it
                            past_reminders.borrow_mut().insert(reminder_id);
                            println!("Marked past reminder: {}", reminder.name);
                        }
                    } else {
                        println!("Failed to parse reminder time for: {}", reminder.name);
                    }
                }
            } else {
                println!("Failed to read reminders");
            }
            
            gtk::glib::ControlFlow::Continue
        }
    ));

    let app = RelmApp::new("Rewind");
    app.run::<AppModel>(0);
}



use gtk::glib::clone;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, WidgetExt, PopoverExt, EntryExt, EditableExt,};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

struct AppModel {
    counter: i8,
    main_window: gtk::Window,
}

#[derive(Debug)]
enum AppMsg {
    NewReminder,
    FinalizeReminder(String),
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
        let model = AppModel { 
            counter: counter.try_into().unwrap(), 
            main_window: window.clone() 
        };

        // Header bar setup
        let header = gtk::HeaderBar::new();
        let menu_button = gtk::MenuButton::new();
        let new_tracked = gtk::Button::new();
        
        header.pack_end(&menu_button);
        header.pack_end(&new_tracked);
        window.set_titlebar(Some(&header));
        
        menu_button.set_icon_name("open-menu-symbolic");
        new_tracked.set_icon_name("list-add");

        // Menu dropdown setup
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
            
            AppMsg::FinalizeReminder(text) => {
                println!("Reminder created: {}", text);
                // Handle the reminder creation here
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
                
                let reminder_name = gtk::Entry::new();
                reminder_name.set_placeholder_text(Some("What is the Reminder Called?"));
                
                let finalize = gtk::Button::new();
                finalize.set_icon_name("checkmark-symbolic");
                
                finalize.connect_clicked(clone!(
                    #[strong] sender,
                    #[strong] reminder_name,
                    move |_| {
                        let text = reminder_name.text().to_string();
                        sender.input(AppMsg::FinalizeReminder(text));
                    }
                ));
                
                reminderbox.append(&reminder_name);
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
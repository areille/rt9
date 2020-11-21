use glib::signal::SignalHandlerId;
use gtk::prelude::*;
use gtk::{Builder, Window, Button, Label};

static LAYOUT_GLADE: &str = include_str!("layout.glade");

type ButtonInfo = (&'static str, &'static str);

static INPUT_BUTTONS: [ButtonInfo; 9] = [
    ("button1", "1"),
    ("button2", "2"),
    ("button3", "3"),
    ("button4", "4"),
    ("button5", "5"),
    ("button6", "6"),
    ("button7", "7"),
    ("button8", "8"),
    ("button9", "9"),
];

pub struct App {
    builder: Builder,
    window: Window,
}

impl App {
    pub fn new() -> App {
        let builder = Builder::new_from_string(LAYOUT_GLADE);
        let window: Window = builder.get_object("window").unwrap();

        App { builder, window }
    }

    pub fn init(&self) {
        self.window.show_all();
        self.setup_inputs();
    }

    pub fn setup_new() -> App {
        let app = App::new();
        app.init();
        app
    }

    pub fn on_close<F: Fn() -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId {
        self.window.connect_delete_event(move |_, _| f())
    }

    pub fn setup_inputs(&self) {
        let input: Label = self.builder.get_object("label2").expect("unable to find label 2");
        // Setup regular buttons
        for &(id, text) in &INPUT_BUTTONS {
            let input = input.clone();
            let button: Button = self.builder.get_object(id).expect("unable to find this object");
            button.connect_clicked(move |_| insert_text(&input, text));
        }
        // Setup delete button
        let delete_button: Button = self.builder.get_object("button10").expect("Unable to find delete button");
        delete_button.connect_clicked(move |_| delete_last_input_character(&input));
    }
}

fn insert_text(input: &Label, text: &str) {
    let mut new = input.get_text().unwrap().to_string();
    new.push_str(text);
    input.set_text(new.as_str());
}

fn delete_last_input_character(input: &Label) {
    let mut new = input.get_text().unwrap().to_string();
    new.pop();
    input.set_text(new.as_str());
}

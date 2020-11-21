use glib::signal::SignalHandlerId;
use gtk::prelude::*;
use gtk::{Builder, Window};

static LAYOUT_GLADE: &str = include_str!("layout.glade");

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
    }

    pub fn setup_new() -> App {
        let app = App::new();
        app.init();
        app
    }

    pub fn on_close<F: Fn() -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId {
        self.window.connect_delete_event(move |_, _| f())
    }
}

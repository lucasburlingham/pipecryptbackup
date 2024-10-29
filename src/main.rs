use gtk::prelude::*;
use gtk::{ glib, Application, Button, ApplicationWindow };

pub mod backup;
pub mod crypto;

const APP_ID: &str = "me.lucasburlingham.pipecryptbackup";

fn main() -> glib::ExitCode {
    // From https://gtk-rs.org/gtk4-rs/stable/latest/book/hello_world.html
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Start initial backup")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(
        |_button| {
            // Start the backup process (collect user information, write to ini file, etc.)
            backup::init();
        }
    );

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Pipecrypt Backup")
        .child(&button)
        .build();

    // Present window
    window.present();
}

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Image, Label, Orientation};

fn main() {
    // Initialize GTK application
    let app = Application::builder()
        .application_id("com.example.magiskwaydroid")
        .build();

    app.connect_activate(|app| {
        // Create the main application window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("magiskWaydroid")
            .default_width(600)
            .default_height(400)
            .build();

        // Create a vertical box for the main layout
        let main_vbox = Box::new(Orientation::Vertical, 10);
        main_vbox.set_margin_top(20);
        main_vbox.set_margin_bottom(20);
        main_vbox.set_margin_start(20);
        main_vbox.set_margin_end(20);

        // Create a horizontal box for the image and text
        let header_hbox = Box::new(Orientation::Horizontal, 5);

        // Add the program image (small size)
        let program_image = Image::builder()
            .file("res/magisk.png") // Image path
            .build();
        program_image.set_pixel_size(20); // Adjust size to be smaller

        // Add the program name
        let title_label = Label::builder()
            .label("magiskWaydroid")
            .halign(gtk::Align::Start)
            .build();

        // Add the image and text to the header box
        header_hbox.append(&program_image);
        header_hbox.append(&title_label);

        // Add a spacer to push buttons down
        let spacer = Box::new(Orientation::Vertical, 0);
        spacer.set_vexpand(true);

        // Create a horizontal box for the buttons
        let button_box = Box::new(Orientation::Horizontal, 10);
        button_box.set_halign(gtk::Align::Center);

        // Create the "Install" button
        let install_button = Button::builder()
            .label("Install")
            .width_request(80)
            .height_request(30)
            .build();

        // Create the "Remove" button
        let remove_button = Button::builder()
            .label("Remove")
            .width_request(80)
            .height_request(30)
            .build();

        // Create the "Reset" button
        let reset_button = Button::builder()
            .label("Reset")
            .width_request(80)
            .height_request(30)
            .build();

        // Add click event handlers for buttons
        install_button.connect_clicked(|_| {
            println!("Install button clicked!");
        });

        remove_button.connect_clicked(|_| {
            println!("Remove button clicked!");
        });

        reset_button.connect_clicked(|_| {
            println!("Reset button clicked!");
        });

        // Add buttons to the horizontal box
        button_box.append(&install_button);
        button_box.append(&remove_button);
        button_box.append(&reset_button);

        // Assemble the layout
        main_vbox.append(&header_hbox); // Add the image and text header
        main_vbox.append(&spacer); // Add spacer to push buttons down
        main_vbox.append(&button_box); // Add the button box

        // Add the vertical box to the window
        window.set_child(Some(&main_vbox));

        // Show the window
        window.show();
    });

    // Run the application
    app.run();
}

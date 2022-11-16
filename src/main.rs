use backlight_popup;
use backlight_popup::message;
use backlight_popup::utilities;

use gtk;
use gtk::gdk;
use gtk::glib;
use gtk::prelude::{
    ApplicationExt,
    ApplicationExtManual,
    ContainerExt,
    LabelExt,
    WidgetExt,
};

use std::borrow;
use std::process;
use std::time;

const DEFAULT_CONFIG: backlight_popup::Config = backlight_popup::Config {
    accent_color: borrow::Cow::Borrowed("#00FFFF"),
    refresh_interval: time::Duration::from_millis(100),
    window_opacity: 1.0,
};

fn main() {
    let application = gtk::Application::builder()
        .application_id("segv.backlight-popup")
        .build();

    // NOTE: unsure if connect_activate should be set as it is called everytime the binary is
    // executed

    application.connect_startup(|application: &gtk::Application| {
        let config = match backlight_popup::Config::load(DEFAULT_CONFIG) {
            Err(error) => {
                eprintln!("Error (while loading config): {}", error);
                process::exit(1);
            }
            Ok(config) => config,
        };

        let window = gtk::ApplicationWindow::builder()
            .application(application)
            // TODO: make height and width configurable
            .default_height(250)
            .default_width(250)
            .title("Backlight Popup")
            .type_(gtk::WindowType::Popup)
            .opacity(config.window_opacity)
            .window_position(gtk::WindowPosition::CenterAlways)
            .build();

        let message_handler = match message::MessageHandler::setup() {
            Err(error) => {
                eprintln!("Error (while setting up message handler): {}", error);
                process::exit(1);
            }
            Ok(config) => config,
        };

        if let Some(screen) = WidgetExt::screen(&window) {
            match utilities::enable_rgba_visual(&window, &screen) {
                // The popup is functional even without opacity, so not throwing an error.
                Err(error) => eprintln!("Warning: {}", error),
                Ok(_) => (),
            }
        }

        window.connect_screen_changed(
            |window: &gtk::ApplicationWindow, new_screen: Option<&gdk::Screen>| {
                if let Some(screen) = new_screen {
                    match utilities::enable_rgba_visual(window, screen) {
                        Err(error) => eprintln!("Warning: {}", error),
                        Ok(_) => (),
                    }
                }
            },
        );

        let backlight_percentage = match utilities::get_backlight_percentage() {
            Err(error) => {
                eprintln!(
                    "Error (while trying to retrieve output from xbacklight): {}",
                    error
                );
                process::exit(1);
            }
            Ok(percentage) => percentage,
        };
        // TODO: make font size configurable
        let label = gtk::Label::builder()
            .justify(gtk::Justification::Center)
            .label(&format!(
                "<span color='{}' font='24' weight='heavy'>{}%</span>",
                config.accent_color, backlight_percentage
            ))
            .use_markup(true)
            .build();
        window.add(&label);

        // TODO: make initial state configurable
        window.show_all();

        glib::timeout_add_local(
            config.refresh_interval,
            move || {
                match message_handler.message() {
                    message::DRAW => {
                        let backlight_percentage = match utilities::get_backlight_percentage() {
                            Err(error) => {
                                eprintln!(
                                    "Error (while trying retrive output from xbacklight): {}",
                                    error
                                );
                                process::exit(1);
                            }
                            Ok(percentage) => percentage,
                        };
                        label.set_markup(&format!(
                            "<span color='{}' font='24' weight='heavy'>{}%</span>",
                            config.accent_color, backlight_percentage
                        ));
                    }
                    message::HIDE => {
                        window.hide();
                        message_handler.set_message(message::NONE);
                    }
                    message::NONE => {}
                    message::SHOW => {
                        window.show_all();
                        message_handler.set_message(message::DRAW);
                    }
                    // Message is an usize (convenient) instead of an enum.
                    _ => unreachable!(),
                }

                glib::Continue(true)
            },
        );
    });

    application.run();
}

use fltk::{
    app,
    button::*,
    group::{Flex, Group, Tabs},
    image::PngImage,
    input::Input,
    menu::{Choice, MenuBar, MenuButton},
    output::Output,
    prelude::{GroupExt, InputExt, MenuExt, WidgetBase, WidgetExt, WindowExt},
    window::Window,
};
use fltk::{
    dialog,
    enums::{Event, FrameType, Key, Shortcut},
    image, menu,
};
use fltk::{enums::*, frame::Frame, group::*, menu::*, prelude::*, window::*};
use fltk::{prelude::*, *};
use fltk_theme::{color_themes, ColorTheme};
use fltk_theme::{widget_themes, ThemeType, WidgetTheme};
pub mod appmenu;
pub mod events;
pub mod layout;
pub mod elements;
use crate::layout::layout_main;
use  crate::events::Message;

mod constants;
use constants::constants::{WIN_WIDTH, WIN_HEIGHT, MENU_HEIGHT,LEFT_WIDTH,RIGHT_WIDTH,LEFT_TOP_HEIGHT,PANEL_PADDING,LOGO_WIDTH};



fn main() {
    // Initialize the application
    let app = app::App::default();

    // Apply a custom color theme to the application
    let theme = ColorTheme::new(color_themes::TAN_THEME);
    theme.apply();

    // Apply a custom widget theme, here using the Aero style
    let widget_theme = WidgetTheme::new(ThemeType::Aero);
    widget_theme.apply();

    // Optionally, set the application's widget scheme to the default. This line is commented out and not used.
    // app::set_scheme(app::Scheme::Base);

    // Set the global font size for all widgets in the application
    app::set_font_size(12);

    // Create the main window with specified width and height, set its title, and center it on the screen
    let mut window = Window::default()
        .with_size(WIN_WIDTH, WIN_HEIGHT)
        .with_label("TaLTac 4")
        .center_screen();

    // Create a channel for sending and receiving custom message types
    let (s, r) = app::channel::<Message>();

    // Setup the application's menu bar and other UI components, passing the sender part of the channel to enable event handling
    appmenu::build_menu(s); // Build the menu bar
    elements::element_logo();                       // Add a logo to the window
    elements::element_language_selector(s);       // Add a language selector, potentially sending messages on change
    layout_main(s);                   // Populate the rest of the window's content

    // Make the window resizable
    window.make_resizable(true);

    // Finalize the window setup
    window.end();

    // Display the window
    window.show();

    // Enter the application's event loop, listening for messages sent from the menu bar (or other UI components)
    // and handling them as they are received
    while app.wait() {
        if let Some(msg) = r.recv() {
            // Handle messages received from the channel, e.g., menu item selections
            events::handle_menu_messages(msg);
        }
    }
}

 

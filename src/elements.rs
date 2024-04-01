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

use crate::events::Message;
use crate::constants::constants::{WIN_WIDTH, WIN_HEIGHT, MENU_HEIGHT,LEFT_WIDTH,RIGHT_WIDTH,LEFT_TOP_HEIGHT,PANEL_PADDING,LOGO_WIDTH};

///ELEMENTS
pub fn element_language_selector(s: app::Sender<Message>) {
    let mut choice = Choice::new(WIN_WIDTH - 100, 0, 100, 30, None);
    choice.add_choice("Italiano | English");
    choice.set_value(0);
    choice.set_callback(move |_| {
        s.send(Message::ComboBox_LanguageChanged);
    });
}

pub fn element_text_aligned_left(text: &str) {
    let _frame3 = Frame::new(0, 400, LEFT_WIDTH, 30, text)
        .set_align(enums::Align::Left | enums::Align::Inside);
}

pub fn element_logo() {
    // Load the image
    let mut image = PngImage::load("./assets/logo.png").unwrap();
    image.scale(LOGO_WIDTH, MENU_HEIGHT+10, true, true);
    // Create a frame to hold the image
    let mut frame = Frame::new(0, 0, LOGO_WIDTH, MENU_HEIGHT+10, "");
    frame.set_image(Some(image));
}
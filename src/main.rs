use fltk::{
    app,
    image::PngImage,
    button::*,
    group::{Flex, Tabs},
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
use crate::appmenu::Message;

//WINDOW PARAMETERS
const WIN_WIDTH: i32 = 1000;
const WIN_HEIGHT: i32 = 900;
const LEFT_WIDTH: i32 = 300;
const MENU_HEIGHT: i32 = 40;
const RIGHT_WIDTH: i32 = WIN_WIDTH - LEFT_WIDTH;
const LOGO_WIDTH:i32=180;
fn main() {
 
    //build app and apply theme
    let app = app::App::default();
    let theme = ColorTheme::new(color_themes::TAN_THEME);
    theme.apply();
    let widget_theme = WidgetTheme::new(ThemeType::Aero);
    widget_theme.apply();
    app::set_font_size(12);

    //create window
    let mut window = Window::default()
        .with_size(WIN_WIDTH, WIN_HEIGHT)
        .with_label("TaLTac 4")
        .center_screen();

    //create menubar event handler
    let (s, r) = app::channel::<Message>();

    //build window contents
    appmenu::build_menu(s, WIN_WIDTH);
    build_logo();
    build_language_selector();
    build_content(s);

    window.make_resizable(true);
    window.end();
    window.show();

    //listen to menubar events
    while app.wait() {
        if let Some(msg) = r.recv() {
            appmenu::handle_menu_messages(msg);
        }
    }
}
 fn build_logo(){
     // Load the image
     let mut image = PngImage::load("./assets/logo.png").unwrap();
     image.scale(LOGO_WIDTH, MENU_HEIGHT, true, true);
     // Create a frame to hold the image
     let mut frame = Frame::new(0, 0, LOGO_WIDTH, MENU_HEIGHT, "");
     frame.set_image(Some(image));
 }


fn build_content(s: app::Sender<Message>) {
    // Packed widget to hold the containers side by side
    let mut pack = Pack::new(0, 40, LEFT_WIDTH, 260, "");
    pack.set_type(PackType::Horizontal);
    pack.set_spacing(0); // Adjust spacing between the containers if needed

    build_left_panel(s);
    build_right_panel();
    pack.end();

}

fn build_left_panel(s: app::Sender<Message>) {
    let mut pack = Pack::new(0, 40, LEFT_WIDTH, 260, "");
    pack.set_type(PackType::Vertical);
    pack.set_spacing(0); // Adjust spacing between the containers if needed

    build_left_top_panel(s);

    build_left_middle_panel();

    build_left_bottom_panel();

    pack.end();
}

fn build_right_panel() {
    let _frame2 = Frame::new(LEFT_WIDTH, 200, RIGHT_WIDTH, 260, "Right Container"); 
}
fn build_left_bottom_panel() {
    let mut pack = Pack::new(0, 40, LEFT_WIDTH, 260, "");
    pack.set_type(PackType::Vertical);
    pack.set_spacing(0); // Adjust spacing between the containers if needed
    build_left_bottom_panel_content();
    // Set the text size
    pack.end();
}
fn build_left_bottom_panel_content() {
    draw_text_aligned_left("Dati");
    draw_text_aligned_left("Nome: Sessione del None");
    draw_text_aligned_left("File: VOC (|type|, Sum occ): (?,?)");
    draw_text_aligned_left("Multi-core: OFF");
}
fn draw_text_aligned_left(text: &str) {
    let _frame3 = Frame::new(0, 400, LEFT_WIDTH, 30, text)
        .set_align(enums::Align::Left | enums::Align::Inside);
}
fn build_left_middle_panel() {
    let mut tabs = Tabs::new(0, MENU_HEIGHT, LEFT_WIDTH, 300, "");
    let mut flex = Flex::default_fill().with_label("Corpus\t\t").row();
    let mut col = Flex::default().column();
    flex.fixed(&col, LEFT_WIDTH);
    col.set_pad(10);
    col.set_margin(10);


    col.end();
    flex.end();

    let flex2 = Flex::default_fill().with_label("Datasets\t\t").row();
    let mut col2 = Flex::default().column();
     col2.set_pad(10);
    col2.set_margin(10);


    col2.end();
    flex2.end();

    tabs.end();
    tabs.auto_layout();
}
fn build_left_top_panel(s: app::Sender<Message>) {
    let mut tabs = Tabs::new(0, MENU_HEIGHT, LEFT_WIDTH, 300, "");
    let mut flex = Flex::default_fill().with_label("Sessioni\t\t").row();
  flex.set_margin(10);
    draw_left_top_panel_tab1(s);

    flex.end();

    let flex2 = Flex::default_fill().with_label("Risorsi\t\t").row();
    flex2.end();

    let flex3 = Flex::default_fill().with_label("Mie risorse\t\t").row();
    flex3.end();

    tabs.end();
    tabs.auto_layout();
}

fn draw_left_top_panel_tab1(s: app::Sender<Message>) {
    //let _frame1 = Frame::new(0, 40, 200, 80, "Top Container");
    let mut pack = Pack::new(0, 40, LEFT_WIDTH, 260, "");
    pack.set_type(fltk::group::PackType::Vertical);
    pack.set_spacing(10); // Sets the spacing between widgets

    let mut btn1 = button::Button::new(0, 0, LEFT_WIDTH, 40, "Crea une Nuova Sessione");
     // Set a callback for the button
     btn1.set_callback(move |_| {
        s.send(Message::Button_NuovaSessione);
    });
    let mut btn2 = button::Button::new(0, 0, LEFT_WIDTH, 40, "Cancella sessione");
    btn2.set_callback(move |_| {
        s.send(Message::Button_CancellaSessione);
    });
    draw_text_aligned_left("Sessione:");
    let mut choice = menu::Choice::new(50, 100, LEFT_WIDTH, 30, None);
    choice.add_choice("Session: 05-03-2024 10:35 | 06-03-2024 14:22 ");
choice.set_value(0);
    pack.end();
}
fn build_language_selector(){
    let mut choice = menu::Choice::new(WIN_WIDTH-100, 0, 100, 30, None);
    choice.add_choice("Italiano");
    choice.set_value(0); 
    
}

use fltk::{
    app,
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
const MENU_HEIGHT: i32 = 30;
const RIGHT_WIDTH: i32 = WIN_WIDTH - LEFT_WIDTH;


fn main() {
 
    //build app and apply theme
    let app = app::App::default();
    let theme = ColorTheme::new(color_themes::GRAY_THEME);
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
    build_content();

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



fn build_content() {
    // Packed widget to hold the containers side by side
    let mut pack = Pack::new(0, 40, LEFT_WIDTH, 260, "");
    pack.set_type(PackType::Horizontal);
    pack.set_spacing(0); // Adjust spacing between the containers if needed

    build_left_panel();
    // Second container
    let _frame2 = Frame::new(LEFT_WIDTH, 200, RIGHT_WIDTH, 260, "Right Container");
    pack.end();
}

fn build_left_panel() {
    let mut pack = Pack::new(0, 40, LEFT_WIDTH, 260, "");
    pack.set_type(PackType::Vertical);
    pack.set_spacing(0); // Adjust spacing between the containers if needed

    draw_left_top_panel();

    build_left_middle_panel();

    build_left_bottom_panel();

    pack.end();
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
    let mut tabs2 = Tabs::new(0, MENU_HEIGHT, LEFT_WIDTH, 400, "");
    let mut flex1 = Flex::default_fill().with_label("Corpus\t\t").row();
    let mut col = Flex::default().column();
    flex1.fixed(&col, 160);
    col.set_pad(10);
    col.set_margin(10);

    //let _frame1 = Frame::new(0, 40, 200, 80, "Top Container");
    let mut frame2_pack = Pack::new(0, 40, LEFT_WIDTH, 260, "");
    frame2_pack.set_type(fltk::group::PackType::Vertical);
    frame2_pack.set_spacing(10); // Sets the spacing between widgets

    frame2_pack.end();

    col.end();
    flex1.end();

    let flex2 = Flex::default_fill().with_label("Datasets\t\t").row();
    flex2.end();

    tabs2.end();
    tabs2.auto_layout();
}
fn draw_left_top_panel() {
    let mut tabs = Tabs::new(0, MENU_HEIGHT, LEFT_WIDTH, 300, "");
    let mut flex = Flex::default_fill().with_label("Sessioni\t\t").row();
    let mut col = Flex::default().column();
    flex.fixed(&col, LEFT_WIDTH);
    col.set_pad(10);
    col.set_margin(10);

    draw_left_top_panel_tab1();

    col.end();
    flex.end();

    let grp2 = Flex::default_fill().with_label("Risorsi\t\t").row();
    grp2.end();

    let grp3 = Flex::default_fill().with_label("Mie risorse\t\t").row();
    grp3.end();

    tabs.end();
    tabs.auto_layout();
}

fn draw_left_top_panel_tab1() {
    //let _frame1 = Frame::new(0, 40, 200, 80, "Top Container");
    let mut pack = Pack::new(0, 40, LEFT_WIDTH, 260, "");
    pack.set_type(fltk::group::PackType::Vertical);
    pack.set_spacing(10); // Sets the spacing between widgets

    let mut btn1 = button::Button::new(0, 0, LEFT_WIDTH, 40, "Crea une Nuova Sessione");
    let mut btn2 = button::Button::new(0, 0, LEFT_WIDTH, 40, "Cancella sessione");
    draw_text_aligned_left("Sessione:");
    let mut choice = menu::Choice::new(50, 100, LEFT_WIDTH, 30, None);
    choice.add_choice("Session: 05-03-2024 10:35 | 06-03-2024 14:22 ");
    pack.end();
}

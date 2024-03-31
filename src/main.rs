use fltk::{
    app,
    button::*,
    group::{Flex, Tabs},
    input::Input,
    menu::{Choice, MenuButton, MenuBar},
    output::Output,
    prelude::{GroupExt, InputExt, MenuExt, WidgetBase, WidgetExt, WindowExt},
    window::Window,
};
use fltk::{prelude::*, *};
use fltk::{ enums::*, frame::Frame, group::*, menu::*, prelude::*, window::*};

use fltk_theme::{ColorTheme, color_themes};
use fltk_theme::{widget_themes, WidgetTheme, ThemeType};
use fltk::{
    dialog,
    enums::{Event, FrameType, Key, Shortcut},
    image, menu,
};
pub mod utility;
use crate::utility::{ Message};


const WIN_WIDTH: i32 = 1000;
const WIN_HEIGHT: i32 = 600;
const LEFT_WIDTH: i32 = 600;
const MENU_HEIGHT: i32 = 30;

const RIGHT_WIDTH: i32 = WIN_WIDTH - LEFT_WIDTH;

fn draw_menu(){
    let (s, r) = app::channel::<Message>();

    let mut menu_bar = MenuBar::new(0, 0, WIN_WIDTH, 30, None);
    menu_bar.add_emit(
        "File\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );
    menu_bar.add_emit(
        "Analisi\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );
    menu_bar.add_emit(
        "Impostationi\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );
    menu_bar.add_emit(
        "Info\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );
    menu_bar.end();
}
fn draw_gallery() {
  



// Packed widget to hold the containers side by side
let mut pack = Pack::new(0, 40, 400, 260, "");
pack.set_type(PackType::Horizontal);
pack.set_spacing(0); // Adjust spacing between the containers if needed

    let mut leftpack = Pack::new(0, 40, 400, 260, "");
    leftpack.set_type(PackType::Vertical);
    leftpack.set_spacing(0); // Adjust spacing between the containers if needed


    let mut tabs = Tabs::new(0, MENU_HEIGHT, LEFT_WIDTH, 300, "");
    //let mut tab = Tabs::default_fill();
    let mut grp1 = Flex::default_fill().with_label("Sessioni\t\t").row();
        let mut col = Flex::default().column();
            grp1.fixed(&col, 160);
            col.set_pad(10);
            col.set_margin(10);

            //let _frame1 = Frame::new(0, 40, 200, 80, "Top Container");
            let mut frame1_pack =  Pack::new(0, 40, 400, 260, "Left 1");
                frame1_pack.set_type(fltk::group::PackType::Vertical);
                frame1_pack.set_spacing(10); // Sets the spacing between widgets

                let mut btn1 = button::Button::new(0, 0, 200, 40, "Crea une Nuova Sessione");
                let mut btn2 = button::Button::new(0, 0, 200, 40, "Cancella sessione");
                let mut lbl = frame::Frame::new(0, 0, 200, 40, "Sessione:");
                let mut choice = menu::Choice::new(50, 100, 100, 30, None);
                choice.add_choice("Session | 05-03-2024 10:35 | 06-03-2024 14:22 ");
            frame1_pack.end(); 


            col.end();
        grp1.end();

    let grp2 = Flex::default_fill().with_label("Risorsi\t\t").row();
    grp2.end();

    let grp3 = Flex::default_fill().with_label("Mie risorse\t\t").row();
    grp3.end();

    tabs.end();
    tabs.auto_layout();




   
    let mut tabs2 = Tabs::new(0, MENU_HEIGHT, LEFT_WIDTH, 300, "");
    //let mut tab = Tabs::default_fill();
    let mut grp1 = Flex::default_fill().with_label("Sessioni\t\t").row();
        let mut col = Flex::default().column();
            grp1.fixed(&col, 160);
            col.set_pad(10);
            col.set_margin(10);

            //let _frame1 = Frame::new(0, 40, 200, 80, "Top Container");
            let mut frame2_pack =  Pack::new(0, 40, 400, 260, "Left 1");
                frame2_pack.set_type(fltk::group::PackType::Vertical);
                frame2_pack.set_spacing(10); // Sets the spacing between widgets

              

            frame2_pack.end(); 


            col.end();
        grp1.end();

    let grp2 = Flex::default_fill().with_label("Risorsi\t\t").row();
    grp2.end();

    let grp3 = Flex::default_fill().with_label("Mie risorse\t\t").row();
    grp3.end();

    tabs2.end();
    tabs2.auto_layout();



    let _frame3 = Frame::new(0, 400, 200, 80, "Bottom Container");


    leftpack.end();

// Second container
let _frame2 = Frame::new(LEFT_WIDTH, 200, RIGHT_WIDTH, 260, "Right Container");

pack.end();



}



fn main() {
    let app = app::App::default();
    let theme = ColorTheme::new(color_themes::GRAY_THEME);
    theme.apply();
    let widget_theme = WidgetTheme::new(ThemeType::Aero);
    widget_theme.apply();
    let mut wind = Window::default()
        .with_size(WIN_WIDTH,WIN_HEIGHT)
        .with_label("TaLTac 4")
        .center_screen();   

        draw_menu();
        draw_gallery();



    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}
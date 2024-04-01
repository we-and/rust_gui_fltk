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
use fltk::{enums::*, frame::Frame, group::*, menu::*, prelude::*, window::*};
use fltk::{prelude::*, *};

use fltk::{
    dialog,
    enums::{Event, FrameType, Key, Shortcut},
    image, menu,
};
use fltk_theme::{color_themes, ColorTheme};
use fltk_theme::{widget_themes, ThemeType, WidgetTheme};
pub mod utility;
use crate::utility::Message;

const WIN_WIDTH: i32 = 1000;
const WIN_HEIGHT: i32 = 900;
const LEFT_WIDTH: i32 = 300;
const MENU_HEIGHT: i32 = 30;

const RIGHT_WIDTH: i32 = WIN_WIDTH - LEFT_WIDTH;

fn draw_menu() {
    let (s, r) = app::channel::<Message>();



    let mut menu_bar = MenuBar::new(0, 0, WIN_WIDTH, 30, None);
    menu_bar.add_emit(
        "&File/Corpus Nuovo\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );
    menu_bar.add_emit(
        "&File/Corpus esistente\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );    menu_bar.add_emit(
        "&File/Scan + Parse Alfabeto\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );    menu_bar.add_emit(
        "&File/Scan + Parse Separatori\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );

    menu_bar.add_emit(
        "&File/Importa/Mie risorse\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );
    menu_bar.add_emit(
        "&File/Importa/Tabella in Dataset\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );

    menu_bar.add_emit(
        "&File/Importa/Lista semplice in Dataset\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );
    menu_bar.add_emit(
        "&File/Esporta/Documenti - tipi",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );

    menu_bar.add_emit(
        "&File/Esporta/Tipi - Variabili",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );

    menu_bar.add_emit(
        "&File/Esporta/Riconstruzione Corpus",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );

    menu_bar.add_emit(
        "&File/Esporta/Subcorpus",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );

    menu_bar.add_emit(
        "&File/Esci\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );
    menu_bar.add_emit(
        "Analisi/Pre-Tratamento\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );
    menu_bar.add_emit(
        "Analisi/Lessicale/Tagging\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );
    menu_bar.add_emit(
        "Analisi/Lessicale/Estrazione di Parole Chiave  \t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );
    menu_bar.add_emit(
        "Analisi/Testuale\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );

    menu_bar.add_emit(
        "Impostationi/Modalità Core/Mono-Core\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );
    menu_bar.add_emit(
        "Impostationi/Modalità Core/Multi-Core\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );
    menu_bar.add_emit(
        "Impostationi/Modalità Schermo\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );
    menu_bar.add_emit(
        "Info/Su TalTac4\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );
    menu_bar.add_emit(
        "Info/Licenza\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );
    menu_bar.add_emit(
        "Info/Manuale Utente\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );
    menu_bar.add_emit(
        "Info/Tutorial\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );
    menu_bar.add_emit(
        "Info/Supporto\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );
    menu_bar.end();
}
fn draw_content() {
    // Packed widget to hold the containers side by side
    let mut pack = Pack::new(0, 40, LEFT_WIDTH, 260, "");
    pack.set_type(PackType::Horizontal);
    pack.set_spacing(0); // Adjust spacing between the containers if needed

    draw_left_panel();
    // Second container
    let _frame2 = Frame::new(LEFT_WIDTH, 200, RIGHT_WIDTH, 260, "Right Container");

    pack.end();
}

fn draw_left_panel() {
    let mut leftpack = Pack::new(0, 40, LEFT_WIDTH, 260, "");
    leftpack.set_type(PackType::Vertical);
    leftpack.set_spacing(0); // Adjust spacing between the containers if needed

    draw_left_top_panel();

    draw_left_middle_panel();

    draw_left_bottom_panel();

    leftpack.end();
}
fn draw_left_bottom_panel() {
    let mut leftpack = Pack::new(0, 40, LEFT_WIDTH, 260, "");
    leftpack.set_type(PackType::Vertical);
    leftpack.set_spacing(0); // Adjust spacing between the containers if needed
    draw_left_bottom_panel_content();
    // Set the text size
   leftpack.end();
}
fn draw_left_bottom_panel_content(){
    drawLeftText("Dati");
    drawLeftText("Nome: Sessione del None");
    drawLeftText("File: VOC (|type|, Sum occ): (?,?)");
    drawLeftText("Multi-core: OFF");
 
 }
fn drawLeftText(text:&str){
    let _frame3 = Frame::new(0, 400, LEFT_WIDTH, 30, text).set_align(enums::Align::Left | enums::Align::Inside );
    
}
fn draw_left_middle_panel() {
    let mut tabs2 = Tabs::new(0, MENU_HEIGHT, LEFT_WIDTH, 400, "");
    //let mut tab = Tabs::default_fill();
    let mut grp1 = Flex::default_fill().with_label("Corpus\t\t").row();
    let mut col = Flex::default().column();
    grp1.fixed(&col, 160);
    col.set_pad(10);
    col.set_margin(10);

        //let _frame1 = Frame::new(0, 40, 200, 80, "Top Container");
        let mut frame2_pack = Pack::new(0, 40, LEFT_WIDTH, 260, "");
        frame2_pack.set_type(fltk::group::PackType::Vertical);
        frame2_pack.set_spacing(10); // Sets the spacing between widgets

        frame2_pack.end();

    col.end();
    grp1.end();

    let grp2 = Flex::default_fill().with_label("Datasets\t\t").row();
    grp2.end();

    tabs2.end();
    tabs2.auto_layout();
}
fn draw_left_top_panel() {
    let mut tabs = Tabs::new(0, MENU_HEIGHT, LEFT_WIDTH, 300, "");
    //let mut tab = Tabs::default_fill();
    let mut grp1 = Flex::default_fill().with_label("Sessioni\t\t").row();
    let mut col = Flex::default().column();
    grp1.fixed(&col, LEFT_WIDTH);
        col.set_pad(10);
        col.set_margin(10);

      draw_left_top_panel_tab1();

        col.end();
    grp1.end();

    let grp2 = Flex::default_fill().with_label("Risorsi\t\t").row();
    grp2.end();

    let grp3 = Flex::default_fill().with_label("Mie risorse\t\t").row();
    grp3.end();

    tabs.end();
    tabs.auto_layout();
}


fn draw_left_top_panel_tab1(){
      //let _frame1 = Frame::new(0, 40, 200, 80, "Top Container");
      let mut frame1_pack = Pack::new(0, 40, LEFT_WIDTH, 260, "");
      frame1_pack.set_type(fltk::group::PackType::Vertical);
      frame1_pack.set_spacing(10); // Sets the spacing between widgets

      let mut btn1 = button::Button::new(0, 0, LEFT_WIDTH, 40, "Crea une Nuova Sessione");
      let mut btn2 = button::Button::new(0, 0, LEFT_WIDTH, 40, "Cancella sessione");
      drawLeftText("Sessione:");
      let mut choice = menu::Choice::new(50, 100, LEFT_WIDTH, 30, None);
      choice.add_choice("Session: 05-03-2024 10:35 | 06-03-2024 14:22 ");
      frame1_pack.end();

}

fn main() {
    let app = app::App::default();
    let theme = ColorTheme::new(color_themes::GRAY_THEME);
    theme.apply();
    let widget_theme = WidgetTheme::new(ThemeType::Aero);
    widget_theme.apply();
    app::set_font_size(12);
    let mut wind = Window::default()
        .with_size(WIN_WIDTH, WIN_HEIGHT)
        .with_label("TaLTac 4")
        .center_screen();

    draw_menu();
    draw_content();

    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}

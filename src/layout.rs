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


use crate::constants::constants::{WIN_WIDTH, WIN_HEIGHT, MENU_HEIGHT,LEFT_WIDTH,RIGHT_WIDTH,LEFT_TOP_HEIGHT,PANEL_PADDING,LOGO_WIDTH};
use crate::events::Message;
use crate::elements::{*};

// Define a function to layout the main part of the UI, which includes left and right panels.
// The function takes a sender for sending messages of type `Message` as an argument.
pub fn layout_main(s: app::Sender<Message>) {
    // Create a new `Pack` widget to arrange child widgets horizontally.
    // The pack is positioned at the height designated for the menu (MENU_HEIGHT) and spans
    // the entire left width (LEFT_WIDTH) of the window. Its height is set to WIN_HEIGHT,
    // accommodating the full height of the window.
    let mut pack = Pack::new(0, MENU_HEIGHT, LEFT_WIDTH, WIN_HEIGHT, "");
    pack.set_type(PackType::Horizontal); // Set the pack's layout to horizontal.
    pack.set_spacing(0); // Set the spacing between contained widgets to 0, indicating no space between them.

    // Begin adding widgets to the pack. Widgets added between `pack.begin()` and `pack.end()`
    // calls will be managed by this pack, arranged according to the pack's settings.
    pack.begin();
    {
        // Layout the left panel of the UI, passing the sender so the panel can send messages.
        layout_left_panel(s);

        // Layout the right panel of the UI. This function does not require the sender,
        // implying it may not need to send messages, or deals with static content.
        layout_right_panel();
    }
    // Finalize adding widgets to the pack. This call is necessary to complete the setup
    // of the pack and ensure proper layout management of the added widgets.
    pack.end();
}


fn layout_left_panel(s: app::Sender<Message>) {
    let mut flex = Flex::default().size_of_parent().column();
    flex.begin();
    {
        layout_left_top_panel(s);
        layout_left_middle_panel();
        layout_left_bottom_panel();
    }
    flex.end();

}

fn layout_right_panel() {
    let _frame2 = Frame::new(LEFT_WIDTH, 200, RIGHT_WIDTH, 260, "Right Container");
}

fn layout_left_top_panel(s: app::Sender<Message>) -> Group {
    let mut group = Group::default().with_size(LEFT_WIDTH, LEFT_TOP_HEIGHT);
    //group.set_frame(enums::FrameType::FlatBox); // Use a flat box as a simple background
    //group.set_color(enums::Color::Red);
    group.begin();
    {
        let mut tabs = Tabs::default().size_of_parent();
        let mut pack = Pack::default()
        .size_of_parent()
            .with_label("Sessioni\t");
        pack.set_type(fltk::group::PackType::Vertical);
        pack.set_spacing(10);
        pack.begin();
        {

            layout_left_top_panel_tab1(s);
        }
        pack.end();
        //flex.end();

        let flex2 = Flex::default_fill().with_label("Risorsi\t\t").row();
        flex2.begin();
        {}
        flex2.end();

        let flex3 = Flex::default_fill().with_label("Mie risorse\t\t").row();
        flex3.begin();
        {}
        flex3.end();

        tabs.end();
        tabs.auto_layout();
    }
    group.end();
    return group;
}

fn layout_left_middle_panel() -> Group {
    let mut group = Group::default().with_size(LEFT_WIDTH, 300);
    //  group.set_frame(enums::FrameType::FlatBox); // Use a flat box as a simple background
    //    group.set_color(enums::Color::Blue);
    group.begin();
    {
        let mut tabs = Tabs::default().with_size(LEFT_WIDTH, 300);

        let mut flex = Flex::default_fill().with_label("Corpus\t\t").with_type(group::PackType::Vertical);
        flex.begin();
        {
                element_text_aligned_left("Corpus here");
        }
        flex.end();

        let flex2 = Flex::default_fill().with_label("Datasets\t\t").column();
        flex2.begin();
        {
            element_text_aligned_left("Datasets here");
        }
        flex2.end();

        tabs.end();
        tabs.auto_layout();
    }
    group.end();

    return group;
}

fn layout_left_bottom_panel() -> Group {
    let mut group = Group::default().with_size(LEFT_WIDTH, 150);
    // group.set_frame(enums::FrameType::FlatBox); // Use a flat box as a simple background
    // group.set_color(enums::Color::Green);
    group.begin();
    {
        let mut pack = Pack::default().with_size(LEFT_WIDTH, 150).with_type(PackType::Vertical);
        pack.begin();
        {
            layout_left_bottom_panel_content();
        }
        pack.end();
    }
    group.end();
    return group;
}

fn layout_left_bottom_panel_content() {
    element_text_aligned_left("Dati");
    element_text_aligned_left("Nome: Sessione del None");
    element_text_aligned_left("File: VOC (|type|, Sum occ): (?,?)");
    element_text_aligned_left("Multi-core: OFF");
}

fn layout_left_top_panel_tab1(s: app::Sender<Message>) {
    let mut group = Group::default().with_size(LEFT_WIDTH, LEFT_TOP_HEIGHT);
    //group.set_frame(enums::FrameType::FlatBox); // Use a flat box as a simple background
    //group.set_color(enums::Color::Green);
    group.begin();
    {
        let mut pack = Pack::new(
            PANEL_PADDING,
            PANEL_PADDING,
            LEFT_WIDTH - 2 * PANEL_PADDING,
            LEFT_TOP_HEIGHT,
            "",
        ).with_type(group::PackType::Vertical);
        pack.set_spacing(10);
        pack.begin();
        {
            let mut btn1 = button::Button::default()
                .with_size(LEFT_WIDTH - 20, 40)
                .with_label("Crea une Nuova Sessione");
            // Set a callback for the button
            btn1.set_callback(move |_| {
                s.send(Message::Button_NuovaSessione);
            });
            let mut btn2 = button::Button::default()
                .with_size(LEFT_WIDTH, 40)
                .with_label("Cancella sessione");
            btn2.set_callback(move |_| {
                s.send(Message::Button_CancellaSessione);
            });
            element_text_aligned_left("Sessione:");
            let mut choice = Choice::default().with_size(LEFT_WIDTH, 30);
            choice.add_choice("Session: 05-03-2024 10:35 | 06-03-2024 14:22 ");
            choice.set_value(0);
        }
        pack.end();
    }
    group.end();
}


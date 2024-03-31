use fltk::dialog;
#[derive(Debug, Copy, Clone)]
pub enum Message {
    Login,
    SendMessage,
    GoToNewest,
    NewestIfToggled,
    Redraw,
    Quit,
}

// credit: https://stackoverflow.com/questions/30811107/how-do-i-get-the-first-character-out-of-a-string#comment83958722_48482196
pub fn car_cdr(s: &str) -> (&str, &str) {
    match s.chars().next() {
        Some(c) => s.split_at(c.len_utf8()),
        None => s.split_at(0),
    }
}

pub fn get_password() -> String {
    
    
    return "a".to_string();
}

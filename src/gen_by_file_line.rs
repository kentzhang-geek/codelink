use clipboard_win::{Clipboard, formats, Getter, Setter, get_clipboard_string, set_clipboard_string};

pub fn from_parameters(file: String, line: String) {
    let mut link = String::from("codelink://") + &file + ":" + &line;
    set_clipboard_string(&link).unwrap();
    println!("write clipboard {:?}", link);
    return;
}

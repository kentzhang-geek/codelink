use clipboard_win::{Clipboard, formats, Getter, Setter, get_clipboard_string, set_clipboard_string};

pub fn from_parameters(file: String, line: String) {
    let mut str_parsed = file.replace("\\", "/");
    let mut link = String::from("codelink://") + &str_parsed + ":" + &line;
    set_clipboard_string(&link).unwrap();
    println!("write clipboard {:?}", link);
    return;
}

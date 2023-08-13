use clipboard_win::{Clipboard, formats, Getter, Setter, get_clipboard_string, set_clipboard_string};
pub fn from_rider() {
    let mut link: String = get_clipboard_string().unwrap();
    link = String::from("codelink://") + &link;
    set_clipboard_string(&link).unwrap();
    println!("write clipboard {:?}", link);
    return;
}

pub fn to_rider() {
    let mut link: String = get_clipboard_string().unwrap();
    link = link.replace("codelink://", "");
    set_clipboard_string(&link).unwrap();
    println!("write clipboard {:?}", link);
    return;
}
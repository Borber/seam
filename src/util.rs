use crate::modle::ShowType;

pub fn match_show_type(t: ShowType) {
    match t {
        ShowType::On(urls) => println!("{}", urls.join("\n\n")),
        ShowType::Off => println!("未开播"),
        ShowType::Error(msg) => println!("{msg}"),
    }
}
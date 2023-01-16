extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<')
.add(b'>').add(b'`');

pub fn get_command(query_string: &str) -> &str {
    if query_string.contains(' ') {
        let index_of_space = query_string.find(' ').unwrap_or(0);
        return &query_string[..index_of_space];
    }
    &query_string
}

pub fn google_search(search_txt: &str) -> String {
    let search = utf8_percent_encode(search_txt, FRAGMENT).to_string();
    let search_url = format!("https://google.com/search?q={}", search);
    search_url
}

pub fn youtube_redirect(search_txt: &str) -> String {
    let link = String::from("https://www.youtube.com/");
    if search_txt == "yt" {
        link
    }
    else if &search_txt[..4] == "yt @" {
        let user_url = format!("{}{}",link,&search_txt[3..]);
        user_url
    }
    else {
        let encoded = youtube_encode(&search_txt[3..]);
        let search_url = format!("{}{}", link, encoded);
        search_url
    }
}

fn youtube_encode(text: &str) -> String {
    let text = text.replace(" ", "+");
    let search = format!("results?search_query={}", text);
    search
}
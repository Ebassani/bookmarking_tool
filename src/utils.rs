extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use regex::Regex;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"')
.add(b'<').add(b'>').add(b'`');
const URL_REGEX: &str =
    r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,4}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)";

fn percent_encoding(text: &str) -> String {
    let text = utf8_percent_encode(text,
         FRAGMENT).to_string();
    text
}

pub fn get_command(command: &str) -> &str {
    if command.contains(' ') {
        let index_of_space = command.find(' ').unwrap_or(0);
        return &command[..index_of_space];
    }
    &command
}

pub fn search_direct(search_txt: &str) -> String {
    let regex: Regex = Regex::new(URL_REGEX).unwrap();
    if regex.is_match(search_txt) {
        let url = direct_url(search_txt);
        url
    }
    else {    
        let search = percent_encoding(search_txt);
        let search_url = format!("https://google.com/search?q={}", search);
        search_url
    }
}

pub fn youtube_redirect(search_txt: &str) -> String {
    let link = String::from("https://youtube.com/");
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

fn direct_url(link: &str) -> String {
    let link = format!("https://{}",&link[2..]);
    link
}

pub fn reddit_redirect(search_txt: &str) -> String {
    let link = String::from("https://reddit.com/");
    if search_txt == "rd" {
        link
    }
    else if &search_txt[..4] == "rd @" {
        let user_url = format!("{}user/{}",link,&search_txt[4..]);
        user_url
    }
    else if &search_txt[..4] == "rd /" {
        let user_url = format!("{}r/{}",link,&search_txt[4..]);
        user_url
    }
    else {
        let encoded = percent_encoding(search_txt);
        let search_url = format!("{}search/?q={}&include_over_18=1"
        , link, encoded);
        search_url
    }
}
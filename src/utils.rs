extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use regex::Regex;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"')
.add(b'<').add(b'>').add(b'`');
const URL_REGEX: &str =
    r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,4}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)";

struct Website {
    id: u16,
    main_page: String,
    command: String,
    scnd_cmd: Vec<String>,
    extend_link: Vec<String>,
    space_encoding: u16,
}

pub fn test(text: &str) -> String {
    
    let youtube_cmds = vec![String::from("yt @")];
    let youtube_links = vec![String::from("{1}{2}"),String::from("{1}results?search_query={2}")];
    let youtube = Website {
        id: 0,
        main_page: String::from("https://youtube.com/"),
        command: String::from("yt"),
        scnd_cmd: youtube_cmds,
        extend_link: youtube_links,
        space_encoding: 1,
    };
    
    if text == youtube.command {
        return youtube.main_page
    }

    let main = &String::from(youtube.main_page).clone()[..];
    for (i, scnd) in youtube.scnd_cmd.iter().enumerate() {
        let size = scnd.len();
        if &text[..size].to_string() == scnd {
            let url = &youtube.extend_link[i].replace("{1}", main);
            let url = &url.replace("{2}", &text[size..]);
            return url.to_string();
        }
    }
    
    let url = youtube.extend_link[youtube.extend_link.len()-1].replace("{1}", main);
    if youtube.space_encoding == 1 {
        let encoded = plus_encode(&text[youtube.command.len()+1..]);
        let url = url.replace("{2}", &encoded[..]);
        url
    }
    else {
        let encoded = percent_encoding(&text[youtube.command.len()+1..]);
        let url = url.replace("{2}", &encoded[..]);
        url
    }
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

fn direct_url(link: &str) -> String {
    let link = format!("https://{}",&link[2..]);
    link
}

fn plus_encode(text: &str) -> String {
    let text = text.replace(" ", "+");
    text
}

fn percent_encoding(text: &str) -> String {
    let text = utf8_percent_encode(text,
         FRAGMENT).to_string();
    text
}

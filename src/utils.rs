extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use regex::Regex;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"')
.add(b'<').add(b'>').add(b'`');
const URL_REGEX: &str = r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,4}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)";

struct Website {
    id: u16,
    main_page: String,
    command: String,
    scnd_cmd: Vec<String>,
    extend_link: Vec<String>,
    space_encoding: u16,
}

fn websites() -> Vec<Website>  {
    let mut websites: Vec<Website> = Vec::new();
    
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

    let reddit_cmds = vec![String::from("rd @"),String::from("rd /")];
    let reddit_links = vec![String::from("{1}user/{2}"),
    String::from("{1}r/{2}"), String::from("{1}search/?q={2}&include_over_18=1")];
    let reddit = Website {
        id: 1,
        main_page: String::from("https://reddit.com/"),
        command: String::from("rd"),
        scnd_cmd: reddit_cmds,
        extend_link: reddit_links,
        space_encoding: 2,
    };
    websites.push(youtube);
    websites.push(reddit);

    websites
}

pub fn web_codes() -> Vec<String> {
    let mut codes: Vec<String> = Vec::new();
    for site in websites() {
        codes.push(site.command);
    }

    codes
}

pub fn test(text: &str, cmd: &str) -> String {
    for site in websites(){
        if site.command == cmd {
            if text == site.command {
                return site.main_page
            }
        
            let main = &String::from(site.main_page).clone()[..];
            for (i, scnd) in site.scnd_cmd.iter().enumerate() {
                let size = scnd.len();
                if &text[..size].to_string() == scnd {
                    let url = &site.extend_link[i].replace("{1}", main);
                    let url = &url.replace("{2}", &text[size..]);
                    return url.to_string();
                }
            }
            
            let url = site.extend_link[site.extend_link.len()-1].replace("{1}", main);
            if site.space_encoding == 1 {
                let encoded = plus_encode(&text[site.command.len()+1..]);
                let url = url.replace("{2}", &encoded[..]);
                return url
            }
            else {
                let encoded = percent_encoding(&text[site.command.len()+1..]);
                let url = url.replace("{2}", &encoded[..]);
                return url
            }
        }
    }
    
    let default = String::from("http://127.0.0.1:8000");
    default
    
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

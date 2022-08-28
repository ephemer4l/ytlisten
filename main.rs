use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::cmp::min;
use std::io::{stdin, stdout, Write};
use std::process::Command;
// use tokio;
use colored::Colorize;

// parse an argument that will enable quick mode to play the first pick
// parse a default argument that takes a youtube link
// parse an argument to make a search string

// #[tokio::main]
fn main() {
    let client = Client::new();
    loop {
        print!("{}", "$ ".truecolor(255, 255, 255));
        stdout().flush().unwrap();
        let mut search = String::new();
        stdin().read_line(&mut search).expect("Filed to read line");
        if &search.trim() == &"q" {
            break;
        } else if &search.trim()[..4] == "http" {
            let split: Vec<&str> = search.trim().split("?v=").collect();
            let id = split[1];
            Command::new("mpv")
                .arg(format!(
                    "https://vid.puffyan.us/latest_version?id={}&itag=251&local=true",
                    id
                ))
                .status()
                .expect("Process failed.");
        } else if &search.trim() == &"" {
            continue;
        } else {
            let response = client
                .get(format!(
                    "https://vid.puffyan.us/search?q={}&type=video",
                    search
                ))
                .send()
                .unwrap()
                .text()
                .unwrap();
            let document = Html::parse_document(&response);
            let title_selector = Selector::parse("div.h-box>a>p").unwrap();
            let titles: Vec<String> = document
                .select(&title_selector)
                .map(|x| x.inner_html())
                .collect();
            let url_selector = Selector::parse("div.pure-u-md-1-4>div.h-box>a").unwrap();
            let urls: Vec<&str> = document
                .select(&url_selector)
                .map(|x| x.value().attr("href").unwrap())
                .collect();
            for number in 0..min(titles.len(), 5) {
                println!(
                    "\n{} {}\n",
                    format!("{}", number).cyan().bold(),
                    titles[number].green()
                )
            }
            print!("Enter selection (n for new search): ");
            stdout().flush().unwrap();
            let mut selection = String::new();
            stdin()
                .read_line(&mut selection)
                .expect("Failed to read line");
            if &selection.trim() == &"n" {
                continue;
            } else if &selection.trim() == &"" {
                continue;
            }
            let selection: usize = selection.trim().parse().expect("Please type a number.");
            let split: Vec<&str> = urls[selection].split("?v=").collect();
            let id = split[1];
            Command::new("mpv")
                .arg(format!(
                    "https://vid.puffyan.us/latest_version?id={}&itag=251&local=true",
                    id
                ))
                .status()
                .expect("Process failed.");
        }
    }
}

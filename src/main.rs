use colored::Colorize;
use scraper::{Html, Selector};
use std::cmp::min;
use std::io::{stdin, stdout, Write};
use std::process::Command;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    loop {
        print!("\x1B[2J\x1B[1;1H");
        print!("{}", "$ ".truecolor(255, 255, 255));
        stdout().flush().unwrap();
        let mut search = String::new();
        stdin().read_line(&mut search).expect("Filed to read line");
        if search.trim() == "q" {
            break;
        } else if &search.trim()[..4] == "http" {
            let split: Vec<&str> = search.trim().split("?v=").collect();
            let id = split[1];
            Command::new("mpv")
                .arg(format!(
                    "https://iv.melmac.space/latest_version?id={}&itag=251&local=true",
                    id
                ))
                .status()
                .expect("Process failed.");
        } else if search.trim() == "" {
            continue;
        } else {
            let response = client
                .get(format!(
                    "https://iv.melmac.space/search?q={}&type=video",
                    search
                ))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            let document = Html::parse_document(&response);
            let selector = Selector::parse("div.video-card-row>a").unwrap();
            let titles: Vec<&str> = document.select(&selector).flat_map(|x| x.text()).collect();
            let urls: Vec<&str> = document
                .select(&selector)
                .map(|x| x.value().attr("href").unwrap())
                .collect();
            for (number, title) in titles.iter().enumerate().take(min(titles.len(), 5)) {
                println!(
                    "\n{} {}\n",
                    format!("{}", number).cyan().bold(),
                    title.green()
                )
            }
            print!("Enter selection (n for new search): ");
            stdout().flush().unwrap();
            let mut selection = String::new();
            stdin()
                .read_line(&mut selection)
                .expect("Failed to read line");
            if (selection.trim() == "n") || (selection.trim() == "") {
                continue;
            }
            let selection: usize = selection.trim().parse().expect("Please type a number.");
            let split: Vec<&str> = urls[selection].split("?v=").collect();
            let id = split[1];
            println!(
                "\n{} {}",
                "Now playing:".magenta().bold(),
                format!("https://youtube.com/watch?v={}\n", id).yellow()
            );
            Command::new("mpv")
                .arg(format!(
                    "https://iv.melmac.space/latest_version?id={}&itag=251&local=true",
                    id
                ))
                .status()
                .expect("Process failed.");
        }
    }
}

use reqwest::blocking::Client;
use scraper::{Html, Selector};
use serde::{Serialize, Deserialize};
use std::{error::Error, thread, time::Duration, fs};

//the struct to hold the scraped data
#[derive(Serialize, Deserialize, Debug)]
struct TravelAdvisory {
    country: String,
    advisory_level: String,
    advisory_text: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    //the list of countries and their correct travel advisory urls
    let countries = vec![
        ("New Zealand", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/NewZealand.html"),
        ("Mexico", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Mexico.html"),
        ("Canada", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Canada.html"),
    ];

    //setting up the http client
    let client = Client::new();
    let mut advisories = Vec::new();

    for (country_name, url) in countries {
        println!("Scraping: {}", url);

        //making the http requests
        let res = client.get(url).send()?;
        let body = res.text()?;
        let document = Html::parse_document(&body);

        //getting the advisory level
        let advisory_div_selector = Selector::parse("div#tsg-rwd-advisories").unwrap();
        let advisory_level = document
            .select(&advisory_div_selector)
            .next()
            .and_then(|div| div.value().attr("class"))
            .map(|class| {
                if class.contains("tsg-rwd-eab-main-frame-blue") {
                    "Level 1"
                } else if class.contains("tsg-rwd-eab-main-frame-standard") {
                    "Level 2"
                } else if class.contains("tsg-rwd-eab-main-frame-orange") {
                    "Level 3"
                } else if class.contains("tsg-rwd-eab-main-frame-red") {
                    "Level 4"
                } else {
                    "Unknown Level"
                }
            })
            .unwrap_or("Unknown Level")
            .to_string();

        //advisory text extraction
        let advisory_text_selector = Selector::parse("div.tsg-rwd-accordion-copy p").unwrap();
        let advisory_text = document
            .select(&advisory_text_selector)
            .map(|e| e.text().collect::<Vec<_>>().join(" "))
            .collect::<Vec<String>>()
            .join("\n");

        //storing the data
        advisories.push(TravelAdvisory {
            country: country_name.to_string(),
            advisory_level,
            advisory_text,
        });

        thread::sleep(Duration::from_secs(2));
    }

    //making sure it is stored to a json
    let json = serde_json::to_string_pretty(&advisories)?;
    println!("{}", json);
    fs::write("advisories.json", json)?;

    println!("Scraping completed. Data saved to advisories.json");
    Ok(())
}

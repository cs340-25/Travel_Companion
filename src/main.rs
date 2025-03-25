use reqwest::blocking::Client;
use scraper::{Html, Selector};
use serde::{Serialize, Deserialize};
use std::error::Error;

// Struct to hold the scraped data
#[derive(Serialize, Deserialize, Debug)]
struct TravelAdvisory {
    country: String,
    advisory_level: String,
    advisory_text: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // List of countries you want to scrape
    let countries = vec![
        "federated-states-of-micronesia",
        "new-zealand",
        // Add more countries as needed
    ];

    // Base URL for the travel advisory site
    let base_url = "https://travel.state.gov/content/travel/en/traveladvisories/";

    // Set up the HTTP client
    let client = Client::new();

    // Iterate over each country and scrape the data
    let mut advisories = Vec::new();
    for country in countries {
        let url = format!("{}{}-travel-advisory.html", base_url, country);
        println!("Scraping: {}", url);
        
        // Make the HTTP GET request
        let res = client.get(&url).send()?;

        // Parse the HTML response
        let body = res.text()?;
        let document = Html::parse_document(&body);

        // Set up the CSS selectors to extract relevant data
        let country_selector = Selector::parse("h1").unwrap();
        let advisory_selector = Selector::parse("div#advisory-level").unwrap();
        let advisory_text_selector = Selector::parse("div#advisory-text").unwrap();

        // Extract country name
        let country_name = document
            .select(&country_selector)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join(" ");

        // Extract advisory level (e.g., "Level 1: Exercise Normal Precautions")
        let advisory_level = document
            .select(&advisory_selector)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join(" ");

        // Extract advisory text (e.g., "This is a sample advisory text...")
        let advisory_text = document
            .select(&advisory_text_selector)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join(" ");

        // Store the data in the advisory struct
        advisories.push(TravelAdvisory {
            country: country_name,
            advisory_level,
            advisory_text,
        });
    }

    // Serialize the data into JSON
    let json = serde_json::to_string_pretty(&advisories)?;
    println!("{}", json);

    // You could save this JSON to a file if needed
    std::fs::write("advisories.json", json)?;

    Ok(())
}

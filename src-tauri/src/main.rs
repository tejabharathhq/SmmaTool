// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[derive(Debug, Serialize)]
struct LeadItem {
    name: String,
    website: String,
    phone: String,
    address:String,
    has_website: bool,
    has_phone: bool,
    total_reviews: String,
    ratings: String,
    url: String,
}

impl LeadItem {
    fn new(
        name: String,
        website: String,
        phone: String,
        address:String,
        has_website: bool,
        has_phone: bool,
        total_reviews: String,
        ratings: String,
        url: String,
    ) -> Self {
        Self {
            name,
            website,
            phone,
            address,
            has_website,
            has_phone,
            total_reviews,
            ratings,
            url,
        }
    }
}

use serde::Serialize;
use thirtyfour::prelude::*;
use tokio::time::Duration;
use std::fs::File;
use std::io::Write;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn scrape(query: &str, max: &str) -> Result<String, String> {
    const CONTAINER_XPATH: &str =
        "//*[@id=\"QA0Szd\"]/div/div/div[1]/div[2]/div/div[1]/div/div/div[1]/div[1]";
    const DETAILS_CONTAINER_XPATH: &str =
        "//*[@id=\"QA0Szd\"]/div/div/div[1]/div[3]/div/div[1]/div/div/div[2]";
    const CLOSE_BUTTON_XPATH: &str =
    "//*[@id=\"QA0Szd\"]/div/div/div[1]/div[3]/div/div[1]/div/div/div[1]/div/div/div[3]/span/button";

    let mut lead_list = Vec::new();
    let mut scraped_elements = Vec::new();
    let mut cycle_count = 0;
    let _count = match max.to_string().parse::<i32>() {
        Ok(count) => count,
        Err(_) => return Err(String::from("Failed to parse max value")),
    };

    let caps = DesiredCapabilities::chrome();
    let driver = match WebDriver::new("http://localhost:9515", caps).await {
        Ok(driver) => driver,
        Err(_) => return Err(String::from("Failed to create WebDriver")),
    };

    // Navigate to the query URL.
    if let Err(_) = driver.goto(query).await {
        return Err(String::from("Failed to navigate to query URL"));
    }

    let container = match driver.find(By::XPath(CONTAINER_XPATH)).await {
        Ok(container) => container,
        Err(_) => return Err(String::from("Failed to find container element")),
    };

    while cycle_count < _count {
        let elements = match container.find_all(By::XPath("div")).await {
            Ok(elements) => elements,
            Err(_) => {
                continue;
            }
        };
        for element in elements {
            if scraped_elements.contains(&element.text().await.unwrap()) {
                continue;
            } else {
                if let Ok(text) = element.text().await {
                    if text.len() > 10 {
                        let mut lead = LeadItem::new(
                            String::from("Empty"),
                            String::from("Empty"),
                            String::from("Empty"),
                            String::from("Empty"),
                            false,
                            false,
                            String::from("Empty"),
                            String::from("Empty"),
                            String::from("Empty"),
                        );

                        if let Err(_) = element.click().await {
                            continue;
                        }

                        let url = driver.current_url().await;

                        match url {
                            Ok(link) => lead.url = link.to_string(),
                            _ => continue,
                        }

                        if let Ok(true) = driver
                            .query(By::XPath(DETAILS_CONTAINER_XPATH))
                            .exists()
                            .await
                        {
                            let details_container =
                                match driver.find(By::XPath(DETAILS_CONTAINER_XPATH)).await {
                                    Ok(container) => container,
                                    Err(_) => {
                                        continue;
                                    }
                                };
                            let delay_duration = Duration::from_secs(1); // Adjust the duration as needed
                            tokio::time::sleep(delay_duration).await;
                            let details_children =
                              /*   match details_container.find_all(By::ClassName("CsEnBe")).await { */
                                match details_container.find_all(By::Css("*")).await {
                                    Ok(children) => children,
                                    Err(_) => {continue;}
                                };
                            println!("Grabbed the children");

                            let business_name = match details_container
                                .query(By::ClassName("DUwDvf"))
                                .first()
                                .await
                            {
                                Ok(element) => element.text().await.unwrap_or_else(|_| {
                                    String::from("Failed to get business name")
                                }),
                                Err(_) => continue,
                            };

                            for detail in details_children {
                                if let Ok(attribute) = detail.attr("aria-label").await {
                                    let aria = attribute
                                        .unwrap_or_else(|| String::from("Nothing in here"));
                                    if aria.contains("Phone:") {
                                        let parts: Vec<&str> = aria.split("Phone:").collect();
                                        if parts.len() > 1 {
                                            lead.phone = parts[1].to_string();
                                            lead.has_phone = true
                                        }
                                    } else if aria.contains("Website:") {
                                        let parts: Vec<&str> = aria.split("Website:").collect();
                                        if parts.len() > 1 {
                                            lead.website = parts[1].to_string();
                                            lead.has_website = true;
                                        }
                                    }
                                    else if aria.contains("Address:") {
                                        let parts: Vec<&str> = aria.split("Address:").collect();
                                        if parts.len() > 1 {
                                            lead.address = parts[1].to_string();
                                        }
                                    }
                                    
                                     else if aria.contains("stars") {
                                        let parts: Vec<&str> = aria.split(" ").collect();
                                        if parts.len() > 1 {
                                            lead.ratings = parts[0].to_string()
                                        }
                                    } else if aria.contains("reviews") {
                                        let parts: Vec<&str> = aria.split(" ").collect();
                                        if parts.len() > 1 {
                                            lead.total_reviews = parts[0].to_string()
                                        }
                                    }

                                    continue;
                                }
                            }

                            if let Err(_) = driver
                                .find(By::XPath(CLOSE_BUTTON_XPATH))
                                .await
                                .unwrap()
                                .click()
                                .await
                            {
                                continue;
                            }

                            //Assigning Lead Name
                            lead.name = business_name;

                            println!("{:?}", lead);
                            lead_list.push(lead);
                        }
                    }
                }
                match element.text().await {
                    Ok(text) => {
                        scraped_elements.push(text);
                    }
                    Err(_) => {
                        continue;
                        // Handle the error as needed
                    }
                }
            };
        }

        match driver
            .execute(
                "arguments[0].scrollTop = arguments[0].scrollHeight;",
                vec![container.to_json().unwrap()],
            )
            .await
        {
            Err(_) => return Err(String::from("Failed to scroll to the bottom of container")),
            _ => (),
        }

        match driver
            .execute(
                "arguments[0].scrollTop = arguments[0].scrollHeight;",
                vec![container.to_json().unwrap()],
            )
            .await
        {
            Err(_) => return Err(String::from("Failed to scroll to the bottom of container")),
            _ => (),
        }
        cycle_count = cycle_count + 1;
    }

    driver.quit().await.unwrap();
    let json = serde_json::to_string(&lead_list);
    match json {
        Ok(data) => {
            let mut data_file = File::create("data.json").expect("creation failed");

            // Write contents to the file
            data_file.write(data.as_bytes()).expect("write failed");

            return Ok(data)
        
        },
        _ => return Ok("[]".to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, scrape])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

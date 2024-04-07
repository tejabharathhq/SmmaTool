// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[derive(Debug)]
struct LeadItem {
    name: String,
    website: String,
    phone: String,
    has_website: bool,
    has_phone: bool,
    total_reviews: String,
    ratings: String,
}

impl LeadItem {
    fn new(
        name: String,
        website: String,
        phone: String,
        has_website: bool,
        has_phone: bool,
        total_reviews: String,
        ratings: String,
    ) -> Self {
        Self {
            name,
            website,
            phone,
            has_website,
            has_phone,
            total_reviews,
            ratings,
        }
    }
}

use thirtyfour::prelude::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn scrape(query: &str, max: &str) -> Result<String, String> {
    let _count = match max.to_string().parse::<i32>() {
        Ok(count) => count,
        Err(_) => return Err(String::from("Failed to parse max value")),
    };
    const CONTAINER_XPATH: &str =
        "//*[@id=\"QA0Szd\"]/div/div/div[1]/div[2]/div/div[1]/div/div/div[1]/div[1]";
    const DETAILS_CONTAINER_XPATH: &str =
        "//*[@id=\"QA0Szd\"]/div/div/div[1]/div[3]/div/div[1]/div/div/div[2]";
    const CLOSE_BUTTON_XPATH: &str =
        "//*[@id=\"QA0Szd\"]/div/div/div[1]/div[3]/div/div[1]/div/div/div[1]/div/div/div[3]/span/button";

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

    let elements = match container.find_all(By::XPath("div")).await {
        Ok(elements) => elements,
        Err(_) => return Err(String::from("Failed to find elements within container")),
    };

    let mut lead_list = Vec::new();

    for element in elements {
        if let Ok(text) = element.text().await {
            if text.len() > 10 {
                let mut lead = LeadItem::new(
                    String::from("Empty"),
                    String::from("Empty"),
                    String::from("Empty"),
                    false,
                    false,
                    String::from("Empty"),
                    String::from("Empty"),
                );

                if let Err(_) = element.click().await {
                    return Err(String::from("Failed to click element"));
                }

                if let Ok(true) = driver.query(By::XPath(DETAILS_CONTAINER_XPATH)).exists().await {
                    let details_container = match driver.find(By::XPath(DETAILS_CONTAINER_XPATH)).await {
                        Ok(container) => container,
                        Err(_) => return Err(String::from("Failed to find details container element")),
                    };

                    let details_children = match details_container.find_all(By::Css("*")).await {
                        Ok(children) => children,
                        Err(_) => return Err(String::from("Failed to find children elements within details container")),
                    };

                    let business_name = match details_container.query(By::ClassName("DUwDvf")).first().await {
                        Ok(element) => element.text().await.unwrap_or_else(|_| String::from("Failed to get business name")),
                        Err(_) => continue,
                        
                    };

                  

                    for detail in details_children {
                        if let Ok(attribute) = detail.attr("aria-label").await {
                            let aria = attribute.unwrap_or_else(|| String::from("Nothing in here"));
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

                            else if aria.contains("stars"){
                                let parts: Vec<&str> = aria.split(" ").collect();
                                if parts.len()> 1{
                                    lead.ratings = parts[0].to_string()
                                }
                            }
                            else if aria.contains("reviews"){
                                let parts: Vec<&str> = aria.split(" ").collect();
                                if parts.len()> 1{
                                    lead.total_reviews = parts[0].to_string()
                                }
                            }
                        }
                    }

                    if let Err(_) = driver.find(By::XPath(CLOSE_BUTTON_XPATH)).await.unwrap().click().await {
                        return Err(String::from("Failed to click close button"));
                    }

                    //Assigning Lead Name

                    lead.name = business_name;

                    println!("{:?}",lead);
                    lead_list.push(lead);
                }
            }
        }
    }

    driver.quit().await.unwrap();

    Ok(String::from("Hello"))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, scrape])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

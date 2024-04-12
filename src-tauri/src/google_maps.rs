use serde::Serialize;
use tauri::{Manager, Window};
use thirtyfour::prelude::*;
use tokio::time::Duration;

use uuid::Uuid;

fn generate_uuid() -> String {
    // Generate a new UUID
    let uuid = Uuid::new_v4();

    // Convert the UUID to a string and return it
    uuid.to_string()
}

#[derive(Debug, Serialize)]
struct LeadItem {
    id: String,
    name: String,
    website: String,
    phone: String,
    address: String,
    has_website: bool,
    has_phone: bool,
    total_reviews: String,
    ratings: String,
    url: String,
    owner_name: String,
    call_completed: bool,
    call_successful: bool,
    location: String,
    status: String,
}

impl LeadItem {
    fn new(
        id: String,
        name: String,
        website: String,
        phone: String,
        address: String,
        has_website: bool,
        has_phone: bool,
        total_reviews: String,
        ratings: String,
        url: String,
        owner_name: String,
        call_completed: bool,
        call_successful: bool,
        location: String,
    ) -> Self {
        Self {
            id,
            name,
            website,
            phone,
            address,
            has_website,
            has_phone,
            total_reviews,
            ratings,
            url,
            owner_name,
            call_completed,
            call_successful,
            location,
            status: String::from("pending"),
        }
    }
}

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// init a background process on the command, and emit periodic events only to the window that used the command
#[tauri::command]
fn init_process(lead: String, window: Window) {
    window
        .emit(
            "lead-scraped",
            Payload {
                message: lead.into(),
            },
        )
        .unwrap();
}

#[tauri::command]
pub async fn scrape(
    query: &str,
    max: &str,
    location: &str,
    window: Window,
) -> Result<String, String> {
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
    let _ = driver.maximize_window().await;
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
                        let id = generate_uuid();
                        let mut lead = LeadItem::new(
                            id,
                            String::from("Empty"),
                            String::from("Empty"),
                            String::from("Empty"),
                            String::from("Empty"),
                            false,
                            false,
                            String::from("Empty"),
                            String::from("Empty"),
                            String::from("Empty"),
                            String::from("Empty"),
                            false,
                            false,
                            String::from(location.to_string()),
                        );

                        if let Err(_) = element.click().await {
                            continue;
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

                            let url = driver.current_url().await;

                            match url {
                                Ok(link) => lead.url = link.to_string(),
                                _ => continue,
                            }
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
                                    } else if aria.contains("Address:") {
                                        let parts: Vec<&str> = aria.split("Address:").collect();
                                        if parts.len() > 1 {
                                            lead.address = parts[1].to_string();
                                        }
                                    } else if aria.contains("stars") {
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

                            /*  println!("{:?}", lead); */
                            let lead_json = serde_json::to_string(&lead);
                            match lead_json {
                                Ok(item) => {
                                    println!("{:?}", item);
                                    init_process(item, window.clone());
                                }
                                _ => println!("failed to send the lead"),
                            }
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
            return Ok(data);
        }
        _ => return Ok("[]".to_string()),
    }
}

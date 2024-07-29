use reqwest::Client;
use std::error::Error;
use regex::Regex;
use serde::Deserialize;
use serenity::all::Timestamp;

#[derive(Deserialize)]
struct Item {
    content: String,
    timestamp: Timestamp
}

#[derive(Debug)]
struct KillInfo {
    killer: String,
    killed: String,
    gun: String,
    distance: f64,
    kill_date: Timestamp,
}

pub async fn collect_messages() -> Result<(), Box<dyn Error>> {
    // Create a reqwest client
    let client = Client::new();

    // Define the URL and headers
    let url = "https://discord.com/api/v9/channels/1114634460428771368/messages?before=1266479311351840890&limit=50";
    let response = client.get(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:128.0) Gecko/20100101 Firefox/128.0")
        .header("Accept", "*/*")
        .header("Accept-Language", "en-US,ru-RU;q=0.8,ru;q=0.5,en;q=0.3")
        .header("Authorization", "token")
        .header("X-Discord-Locale", "ru")
        .header("X-Discord-Timezone", "Europe/Budapest")
        .header("X-Debug-Options", "bugReporterEnabled")
        .header("Alt-Used", "discord.com")
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-origin")
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        // Print the response body
        let body = response.text().await?;

        let items: Vec<Item> = serde_json::from_str(&body)?;
        let kills: Vec<KillInfo> = items.iter()
            .filter_map(|item| extract_kill_info(&item.content, &item.timestamp))
            .collect();

        // Output the list of content strings
        for (i, kill) in kills.iter().enumerate() {

            println!("{:?}: {:?} : {:?} - {:?}, {:?} from {:?}m ",kill.kill_date, i, kill.killer, kill.killed, kill.gun, kill.distance );
        }
        /*        println!("{:?}", content_list);

                println!("Response body: {}", body);*/
    } else {
        // Print error if the request failed
        println!("Error: {}", response.status());
    }

    Ok(())
}

fn extract_kill_info(input: &str, timestamp:&Timestamp) -> Option<KillInfo> {
    // Define the regular expression
    let re = Regex::new(r"\[(.*?)\]\(<[^>]+>\) got killed by \[(.*?)\]\(<[^>]+>\) \(([^,]+), ([\d.]+)m\)").unwrap();


    // Apply the regular expression to the input string
    if let Some(captures) = re.captures(input) {
        // Extract the values using the capture groups and create a KillInfo struct
        let kill_info = KillInfo {
            killer: captures.get(1).map_or("", |m| m.as_str()).to_string(),
            killed: captures.get(2).map_or("", |m| m.as_str()).to_string(),
            gun: captures.get(3).map_or("", |m| m.as_str()).to_string(),
            distance: captures.get(4).map_or("0", |m| m.as_str()).parse().unwrap_or(0.0),
            kill_date: timestamp.clone()
        };

        // Return the KillInfo struct
        Some(kill_info)
    } else {
        // Return None if no match is found
        println!("No match found for: {:?}", input);
        None
    }
}

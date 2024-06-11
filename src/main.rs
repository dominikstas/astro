use reqwest::Error;
use serde::Deserialize;
use std::env;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Fetching astronomy data...");

    let mut comet_data = fetch_nearby_comets().await?;
    let mut space_program_data = fetch_space_programs().await?;

    let comet_choice = get_user_choice("Do you want to see the comet data? (yes/no): ");
    let comet_limit = if comet_choice.to_lowercase() == "yes" {
        let limit = get_user_input("How many comets do you want to see?: ").parse::<usize>().unwrap_or(10);
        let sort_choice = get_user_input("Sort comet data by (1) Distance or (2) Time: ").parse::<u8>().unwrap_or(1);
        if sort_choice == 1 {
            comet_data.sort_by(|a, b| a.miss_distance.as_deref().unwrap_or("0").parse::<f64>().unwrap_or(0.0).partial_cmp(&b.miss_distance.as_deref().unwrap_or("0").parse::<f64>().unwrap_or(0.0)).unwrap());
        } else {
            comet_data.sort_by(|a, b| a.close_approach_date.cmp(&b.close_approach_date));
        }
        limit
    } else {
        0
    };

    let space_program_choice = get_user_choice("Do you want to see the space program data? (yes/no): ");
    let space_program_limit = if space_program_choice.to_lowercase() == "yes" {
        let limit = get_user_input("How many space programs do you want to see?: ").parse::<usize>().unwrap_or(10);
        let sort_choice = get_user_input("Sort space program data by (1) Launch Time: ").parse::<u8>().unwrap_or(1);
        if sort_choice == 1 {
            space_program_data.sort_by(|a, b| a.name.cmp(&b.name)); // Assuming name contains launch time info for sorting
        }
        limit
    } else {
        0
    };

    if comet_limit > 0 {
        println!("\nNearby Comets:");
        for comet in comet_data.into_iter().take(comet_limit) {
            println!(
                "Name: {}, Close Approach Date: {:?}, Miss Distance (km): {:?}",
                comet.name, comet.close_approach_date, comet.miss_distance
            );
        }
    }

    if space_program_limit > 0 {
        println!("\nSpace Programs:");
        for program in space_program_data.into_iter().take(space_program_limit) {
            println!(
                "Name: {}, Description: {:?}, Agency: {:?}",
                program.name, program.description, program.agency
            );
        }
    }

    Ok(())
}

fn get_user_choice(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[derive(Deserialize, Debug)]
struct Comet {
    name: String,
    #[serde(rename = "close_approach_date")]
    close_approach_date: Option<String>,
    #[serde(rename = "miss_distance_kilometers")]
    miss_distance: Option<String>,
}

async fn fetch_nearby_comets() -> Result<Vec<Comet>, Error> {
    let api_key = env::var("NASA_API_KEY").unwrap_or_else(|_| "DEMO_KEY".to_string());
    let url = format!(
        "https://api.nasa.gov/neo/rest/v1/feed?start_date=2024-06-06&end_date=2024-06-13&api_key={}",
        api_key
    );
    let response = reqwest::get(&url).await?.json::<serde_json::Value>().await?;
    let comets = response["near_earth_objects"]
        .as_object()
        .unwrap()
        .values()
        .flat_map(|day| day.as_array().unwrap())
        .filter_map(|c| serde_json::from_value(c.clone()).ok())
        .collect::<Vec<Comet>>();
    Ok(comets)
}

#[derive(Deserialize, Debug)]
struct SpaceProgram {
    name: String,
    description: Option<String>,
    agency: Option<String>,
}

async fn fetch_space_programs() -> Result<Vec<SpaceProgram>, Error> {
    let url = "https://api.spacexdata.com/v4/launches";
    let response = reqwest::get(url).await?.json::<Vec<SpaceProgram>>().await?;
    Ok(response)
}

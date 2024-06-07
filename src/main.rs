use reqwest::Error;
use serde::Deserialize;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Fetching astronomy data...");

    let comet_data = fetch_nearby_comets().await?;
    let star_data = fetch_stars().await?;
    let space_program_data = fetch_space_programs().await?;

    println!("\nNearby Comets:");
    for comet in comet_data {
        println!(
            "Name: {}, Close Approach Date: {}, Miss Distance (km): {}",
            comet.name, comet.close_approach_date, comet.miss_distance
        );
    }

    println!("\nStars:");
    for star in star_data {
        println!(
            "Name: {}, Distance: {} light years, Constellation: {}",
            star.name, star.distance, star.constellation
        );
    }

    println!("\nSpace Programs:");
    for program in space_program_data {
        println!(
            "Name: {}, Description: {}, Agency: {}",
            program.name, program.description, program.agency
        );
    }

    Ok(())
}

#[derive(Deserialize, Debug)]
struct Comet {
    name: String,
    #[serde(rename = "close_approach_date")]
    close_approach_date: String,
    #[serde(rename = "miss_distance_kilometers")]
    miss_distance: String,
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
        .map(|c| serde_json::from_value(c.clone()).unwrap())
        .collect::<Vec<Comet>>();
    Ok(comets)
}

#[derive(Deserialize, Debug)]
struct Star {
    name: String,
    distance: f64,
    constellation: String,
}

async fn fetch_stars() -> Result<Vec<Star>, Error> {
    // Example URL, replace with actual API endpoint
    let url = "https://example.com/api/stars";
    let response = reqwest::get(url).await?.json::<Vec<Star>>().await?;
    Ok(response)
}

#[derive(Deserialize, Debug)]
struct SpaceProgram {
    name: String,
    description: String,
    agency: String,
}

async fn fetch_space_programs() -> Result<Vec<SpaceProgram>, Error> {
    let url = "https://api.spacexdata.com/v4/launches";
    let response = reqwest::get(url).await?.json::<Vec<SpaceProgram>>().await?;
    Ok(response)
}

# Astro

This Rust program fetches data about nearby comets and space programs. It uses asynchronous requests to retrieve data from NASA's Near Earth Object Web Service and SpaceX API.

## Features

- Fetches and displays comet data, including name, close approach date, and miss distance.
- Fetches and displays space program data, including name, description, and agency.
- Allows user to specify the number of comets and space programs to display.
- Provides options to sort comet data by distance and time, and space program data by launch time.

## Getting Started

### Prerequisites

- Rust (https://www.rust-lang.org/tools/install)
- Cargo (comes with Rust)
- NASA API Key (optional, a demo key is used by default)

### Installing

1. Clone the repository
    ```sh
    git clone https://github.com/dominikstas/astro.git
    cd astro
    ```

2. Set up your environment
    ```sh
    export NASA_API_KEY=your_api_key_here
    ```

3. Run the program
    ```sh
    cargo run
    ```

## Usage

Upon running the program, it will prompt you with:

1. Whether you want to see comet data.
2. How many comets you want to see.
3. Whether you want to see space program data.
4. How many space programs you want to see.
5. How you would like to sort the comet data (by distance or time).
6. How you would like to sort the space program data (by launch time).


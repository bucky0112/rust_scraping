# Rust Web Scraper

A simple web scraper built with Rust, demonstrating basic web scraping techniques.

## Overview

This project showcases how to create a web scraper using Rust. It's designed to scrape book information from a sample bookstore website (https://books.toscrape.com/), but can be adapted for other similar scraping tasks.

## Features

- Sends HTTP requests to web pages
- Parses HTML content
- Extracts specific data (book titles, prices, and image URLs)
- Handles errors and edge cases
- Demonstrates Rust's performance and safety features in web scraping

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager, usually comes with Rust)

## Installation

1. Clone the repository

2. Build the project:
   
   For a debug build:
   ```
   cargo build
   ```
   
   For a release build (optimized):
   ```
   cargo build --release
   ```

## Usage

To run the scraper:

1. Using Cargo (builds and runs):
   ```
   cargo run
   ```
   Or for release version:
   ```
   cargo run --release
   ```

2. Running the executable directly:
   
   For macOS/Linux:
   ```
   ./target/debug/rust-web-scraper
   ```
   Or for release version:
   ```
   ./target/release/rust-web-scraper
   ```
   
   For Windows:
   ```
   .\target\debug\rust-web-scraper.exe
   ```
   Or for release version:
   ```
   .\target\release\rust-web-scraper.exe
   ```

Note: The executable name might be different based on your project name in Cargo.toml.

## Customization

To scrape a different website or extract different data:

1. Modify the URL in `src/main.rs`
2. Adjust the CSS selectors in the `select` functions to match the target website's structure
3. Update the data extraction logic as needed

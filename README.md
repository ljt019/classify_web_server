# Classify Web Server

This is a Rust-based web server that provides a user interface for the Classify CLI tool. It allows users to upload sketches through a web form and get classification results.

## Features

- Web interface for sketch upload and classification
- Integration with the Sketch Classifier CLI tool
- Built with Rocket, a fast and secure Rust web framework
- Supports multiple file formats for sketch uploads

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)
- Sketch Classifier CLI tool installed and accessible in the system PATH

## Installation

1. Clone the repository:

   ```
   git clone https://github.com/yourusername/classify_web_server.git
   cd sketch-classifier-webserver
   ```

2. Build the project:
   ```
   cargo build --release
   ```

## Usage

1. Start the server:

   ```
   cargo run --release
   ```

2. Open a web browser and navigate to `http://localhost:8000`

3. Use the web form to upload a sketch image and submit for classification

4. View the classification results displayed on the web page

## Configuration

- The server runs on `localhost:8000` by default. To change this, modify the `Rocket.toml` file or use environment variables as specified in the Rocket documentation.

## API Endpoints

- `GET /`: Serves the main page with the upload form
- `POST /classify`: Handles sketch upload and classification

## Dependencies

- Rocket: Web framework for Rust
- rocket_multipart_form_data: For handling file uploads
- tokio: Asynchronous runtime for Rust

## Acknowledgements

- Built with [Rocket](https://rocket.rs/), a web framework for Rust

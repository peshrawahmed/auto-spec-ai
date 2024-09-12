use serde::Deserialize; // Deserialize is used to convert YAML data into Rust structs.

#[derive(Debug, Deserialize)]
pub struct Config {
    pub routes: Vec<Route>, // List of routes defined in the configuration.
}

#[derive(Debug, Deserialize)]
pub struct Route {
    pub path: String,                  // Path of the API route.
    pub method: String,                // HTTP method for the route (GET, POST, etc.).
    pub controllers: Vec<String>,      // List of controller file paths associated with this route.
    pub other_details: Option<String>, // Optional field for additional details.
}

// Reads and parses the YAML configuration file into a Config struct.
pub fn read_config(file_path: &str) -> Config {
    let content = std::fs::read_to_string(file_path).expect("Failed to read the config file");
    // Converts YAML content to a Config struct.
    serde_yaml::from_str(&content).expect("Failed to parse the YAML file")
}

// Reads the content of a controller file from the given file path.
pub fn read_controller(file_path: &str) -> String {
    std::fs::read_to_string(file_path).expect(&format!(
        "Failed to read the controller file: {}",
        file_path
    ))
}

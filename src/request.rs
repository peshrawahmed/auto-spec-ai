use reqwest::Client;
use serde_json::json;
use std::fs::{self, OpenOptions};

// Cleans up unnecessary escape sequences and markers from the JSON response.
pub fn clean_json_response(response: &str) -> String {
    response
        .replace("\\n", "")
        .replace("\n", "")
        .replace("\\\"", "\"")
        .replace("```json", "")
        .replace("```", "")
}

// Sends a POST request to the Ollama API with the provided route and controller data.
pub async fn send_request(
    route_path: &str,                         // API route path
    route_method: &str,                       // HTTP method (GET, POST, etc.)
    controllers: &[(usize, String, String)],  // List of controller files
    other_details: Option<&String>,           // Optional additional details from the config
    output_data: &mut Vec<serde_json::Value>, // Storage for the API responses
    system_prompt_file: &str,                 // Path to the system prompt file
    ollama_address: &str,                     // Ollama API address
) -> Result<(), reqwest::Error> {
    // Read the system prompt content from the file.
    let system_md_content = fs::read_to_string(system_prompt_file).expect(&format!(
        "Failed to read the system prompt file: {}",
        system_prompt_file
    ));

    // Construct the messages array for the API request.
    let mut messages = vec![
        json!({
            "role": "system",
            "content": system_md_content
        }),
        json!({
            "role": "user",
            "content": format!("route: {} method: {}", route_path, route_method)
        }),
    ];

    // Add each controller's content to the messages array.
    for (index, controller_path, controller_content) in controllers {
        messages.push(json!({
            "role": "user",
            "content": format!("controller {}: \n path: {} \n content: \n {}", index + 1, controller_path, controller_content)
        }));
    }

    // If there are other details, include them in the messages.
    if let Some(details) = other_details {
        messages.push(json!({
            "role": "user",
            "content": details
        }));
    }

    // Construct the API request body.
    let body = json!({
        "model": "llama3.1:8b",
        "stream": false,
        "messages": messages
    });

    // Send the POST request to the Ollama API.
    let client = Client::new();
    let res = client
        .post(&format!("{}/api/chat", ollama_address)) // Dynamically set the Ollama API address
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    // Parse the response body as JSON.
    let response_text = res.text().await?;
    let response_json: serde_json::Value =
        serde_json::from_str(&response_text).expect("Failed to parse response JSON");

    // Extract the message content from the response.
    let message_content = response_json["message"]["content"]
        .as_str()
        .unwrap_or("No content");

    // Clean the message content and add it to the output data.
    let cleaned_message_content = clean_json_response(message_content);
    let response_content = json!({
        "route": route_path,
        "method": route_method,
        "message_content": cleaned_message_content
    });

    output_data.push(response_content);

    Ok(())
}

// Writes the collected output data to a specified file in a formatted JSON style.
pub fn write_output_to_file(output_data: &Vec<serde_json::Value>, output_path: &str) {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_path)
        .expect("Failed to open file");

    serde_json::to_writer_pretty(file, &output_data).expect("Failed to write to output file");
}

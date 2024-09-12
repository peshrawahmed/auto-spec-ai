mod config;
mod request;
use clap::Parser;
use std::collections::HashMap;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    config: Option<String>, // Optional config file path.

    #[clap(short, long, default_value = "")]
    output: String, // Optional output file path.

    #[clap(long)]
    route: Option<String>, // Route path if not using config.

    #[clap(long)]
    method: Option<String>, // HTTP method (GET, POST, etc.) if not using config.

    #[clap(long, value_delimiter = ',')]
    controller: Option<Vec<String>>, // Multiple controller file paths for a single route.

    #[clap(short, long, default_value = "system.md")]
    system_prompt: String, // System prompt file path (default is system.md).

    #[clap(long, default_value = "http://localhost:11434")]
    ollama: String, // Ollama API address (default is localhost).
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();

    let mut output_data = vec![]; // Store responses from the requests.

    // Use config file if provided
    if let Some(config_file) = &args.config {
        let config = config::read_config(config_file);

        let mut controller_contents: HashMap<String, String> = HashMap::new(); // Map to hold controller file content.

        for route in config.routes {
            let mut controllers_data = vec![];
            // Read and store each controller's content
            for (i, controller_path) in route.controllers.iter().enumerate() {
                let content = config::read_controller(controller_path);
                controllers_data.push((i, controller_path.clone(), content.clone()));
                controller_contents.insert(controller_path.clone(), content); // Store controller content for further use.
            }

            // Send request with the route and controller data
            request::send_request(
                &route.path,
                &route.method,
                &controllers_data,
                route.other_details.as_ref(), // Pass other details if available.
                &mut output_data,
                &args.system_prompt,
                &args.ollama,
            )
            .await?;
        }
    }
    // If no config, use provided --route, --method, and --controller arguments
    else if let (Some(route), Some(method), Some(controllers)) =
        (&args.route, &args.method, &args.controller)
    {
        let mut controllers_data = vec![];

        // Read and store controller content from the provided controllers
        for (i, controller_path) in controllers.iter().enumerate() {
            let content = config::read_controller(controller_path);
            controllers_data.push((i, controller_path.clone(), content));
        }

        // Send request with the provided route, method, and controller data
        request::send_request(
            route,
            method,
            &controllers_data,
            None, // No other details provided in this case.
            &mut output_data,
            &args.system_prompt,
            &args.ollama,
        )
        .await?;
    }
    // If neither config nor route/method/controller are provided, exit with error.
    else {
        eprintln!("Error: You must provide either --config or --route, --method, and --controller");
        std::process::exit(1);
    }

    // Output results to file or terminal
    if args.output.is_empty() {
        println!("{}", serde_json::to_string_pretty(&output_data).unwrap()); // Print to terminal if no output file is provided.
    } else {
        request::write_output_to_file(&output_data, &args.output); // Write to the specified output file.
    }

    Ok(())
}

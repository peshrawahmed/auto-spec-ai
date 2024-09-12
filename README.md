# Auto-Spec-AI

Auto-Spec-AI is a command-line tool designed to generate OpenAPI 3.1.0 compatible documentation for any project using Large Language Models (LLMs). It is particularly useful for large projects that lack comprehensive API documentation. Auto-Spec-AI allows developers to either generate documentation via a YAML configuration file or by specifying individual routes, methods, and controllers directly via the CLI.

## Table of Contents

- [Auto-Spec-AI](#auto-spec-ai)
- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
  - [1. Using a Configuration File](#1-using-a-configuration-file)
  - [2. Generating Documentation for a Single Route](#2-generating-documentation-for-a-single-route)
  - [3. Using Multiple Controllers for a Single Route](#3-using-multiple-controllers-for-a-single-route)
- [Configuration](#configuration)
  - [Example Configuration File](#example-configuration-file)
  - [Configuration Fields](#configuration-fields)
  - [How to Use the Configuration File](#how-to-use-the-configuration-file)
- [CLI Arguments](#cli-arguments)
- [System Prompt](#system-prompt)
  - [Using a Custom System Prompt](#using-a-custom-system-prompt)
  - [System Prompt File Format](#system-prompt-file-format)
  - [Default System Prompt](#default-system-prompt)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Overview

Auto-Spec-AI is a CLI-based tool built with Rust that automates the generation of API documentation in the OpenAPI 3.1.0 format. By leveraging Large Language Models (LLMs), Auto-Spec-AI simplifies the process of creating detailed API specifications for projects with extensive microservice architectures.

The tool can be used in two ways:

- By specifying a configuration file in YAML format that defines all routes, methods, and associated controllers.
- By passing individual routes, methods, and controller paths as command-line arguments for generating documentation on-the-fly.

Auto-Spec-AI also allows users to define custom system prompts and integrate with Ollama’s API for advanced language model interactions. Whether you're documenting an entire API or focusing on a single route, Auto-Spec-AI provides a flexible and automated solution to ensure your APIs are well-documented.

## Prerequisites

Before you can use Auto-Spec-AI, ensure that the following dependencies and tools are installed on your system:

### 1. Rust

Auto-Spec-AI is built with Rust, so you need to have the Rust toolchain installed. Follow the instructions below based on your operating system:

#### Mac/Linux:

Run the following command in your terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, ensure Rust is installed by running:

```bash
rustc --version
```

#### Windows:

Download and run the installer from [Rust's official website](https://www.rust-lang.org/tools/install).  
Follow the installation steps and then verify the installation by opening a command prompt and running:

### 2. Ollama API

Ollama is required for Auto-Spec-AI to function. You can either install Ollama locally or use a remote Ollama instance.

- **Ollama Installation for Mac/Linux/Windows**: Please visit the official Ollama website for instructions to install and configure Ollama: [Ollama Website](https://ollama.com)

Auto-Spec-AI uses `llama3.1:8b` model for now. Do not forget to pull the model using ollama

```bash
ollama pull llama3.1:8b
```

## Installation

Follow the steps below to install and set up Auto-Spec-AI on your system.

### 1. Clone the Repository

First, clone the Auto-Spec-AI repository to your local machine:

```bash
git clone https://github.com/peshrawahmed/auto-spec-ai.git
cd auto-spec-ai
```

### 2. Build the Project

Once you have the repository cloned, build the Auto-Spec-AI project in release mode:

```bash
cargo build --release
```

This will generate the binary in the target/release folder.

## Usage

Auto-Spec-AI can be used in two primary ways: through a YAML configuration file or by providing specific routes, methods, and controllers directly via the CLI.

### 1. Using a Configuration File

To generate API documentation using a YAML configuration file, provide the file with the `--config` argument:

```bash
./target/release/auto-spec-ai --config routes.yaml
```

The configuration file should define the routes, methods, and controllers. An example routes.yaml file might look like this:

```yaml
routes:
  - path: '/api/users'
    method: 'get'
    controllers:
      - 'controllers/users_get.rs'
  - path: '/api/users'
    method: 'post'
    controllers:
      - 'controllers/users_post.rs'
```

### 2. Using Command-Line Arguments

You can also generate documentation by directly specifying the route, method, and controller files using command-line arguments:

```bash
./target/release/auto-spec-ai --route "/api/users" --method get --controller controllers/users_get.rs
```

You can specify multiple controller files if needed:

```bash
./target/release/auto-spec-ai --route "/api/users" --method get --controller controllers/users_get.rs,controllers/extra_logic.rs
```

### 3. Custom System Prompt

You can customize the system prompt for LLM by providing your own prompt file using the `--system-prompt` argument. If not provided, the default system.md file will be used.

```bash
./target/release/auto-spec-ai --config routes.yaml --system-prompt custom_system.md
```

### 4. Specifying the Ollama API Address

If you are using a remote Ollama instance or using Ollama on a different port you can specify the API address using the `--ollama` argument:

```bash
./target/release/auto-spec-ai --config routes.yaml --ollama http://remote-ollama-server:11434
```

### 5. Output to a File

To save the generated documentation to a file instead of printing it to the terminal, use the `--output` argument:

```bash
./target/release/auto-spec-ai --config routes.yaml --output api_docs.json
```

## Configuration

Auto-Spec-AI allows you to define API routes, methods, and controllers using a YAML configuration file. This file serves as an input to generate the OpenAPI 3.1.0 compatible documentation for the APIs.

### Example Configuration File

Below is an example of a `routes.yaml` configuration file:

```yaml
routes:
  - path: '/api/users'
    method: 'get'
    controllers:
      - 'controllers/users_get.rs'
  - path: '/api/users'
    method: 'post'
    controllers:
      - 'controllers/users_post.rs'
  - path: '/api/users/:userId'
    method: 'get'
    controllers:
      - 'controllers/users_get_by_id.rs'
    other_details: 'Requires userFetchPermission.'
```

### Configuration Fields

- **path**: The route path for the API (e.g., `/api/users` or `/api/users/:userId`).
- **method**: The HTTP method to be documented for the route (e.g., `get`, `post`, `put`, etc.).
- **controllers**: A list of controller file paths associated with the route. Each file contains the logic for handling the API request.
- **other_details**: (Optional) Any additional information or custom notes for the API route that may be useful for documentation.

### How to Use the Configuration File

To generate documentation using the YAML configuration file, pass it as an argument to the `--config` option:

```bash
./target/release/auto-spec-ai --config routes.yaml
```

This will generate OpenAPI documentation based on the routes, methods, and controllers specified in the file.

## CLI Arguments

Auto-Spec-AI provides several command-line arguments to customize the behavior of the tool. Below is a list of available arguments and their descriptions:

### 1. `--config <file>`

Use this argument to provide a YAML configuration file that defines routes, methods, and controllers. This is the main way to generate API documentation based on a configuration file.

```bash
./target/release/auto-spec-ai --config routes.yaml
```

### 2. `--route <path>`

Specify the API route manually without using a configuration file. This argument is used along with --method and --controller.

```bash
./target/release/auto-spec-ai --route "/api/users"
```

### 3. `--method <http-method>`

Provide the HTTP method (e.g., get, post, put) for the specified route.

```bash
./target/release/auto-spec-ai --route "/api/users" --method get
```

### 4. `--controller <file>`

Provide one or more controller files associated with the specified route and method. Multiple controllers can be provided, separated by commas.

```bash
./target/release/auto-spec-ai --route "/api/users" --method get --controller controller1.rs,controller2.rs
```

### 5. `--system-prompt <file>`

Use this argument to specify a custom system prompt file instead of the default `system.md`.

```bash
./target/release/auto-spec-ai --config routes.yaml --system-prompt custom_system.md
```

### 6. `--ollama <url>`

This argument allows you to specify the Ollama API address. If not provided, the default is `http://localhost:11434`.

```bash
./target/release/auto-spec-ai --config routes.yaml --ollama http://remote-ollama-server:11434
```

### 7. `--output <file>`

Specify a file to save the generated documentation. If not provided, the output will be printed to the terminal.

```bash
./target/release/auto-spec-ai --config routes.yaml --output api_docs.json
```

### 8. `--help`

Displays a help message with a summary of available command-line arguments.

```bash
./target/release/auto-spec-ai --help
```

### 9. `--model <model>`

This argument will allow you to specify the model to use for generating documentation. Currently, only the llama3.1:8b model has been tested and supported. Future versions of Auto-Spec-AI will include support for additional models.

```bash
./target/release/auto-spec-ai --config routes.yaml --model llama3.1:8b
```

## System Prompt

Auto-Spec-AI allows you to customize the system prompt used when interacting with the Ollama API. By default, the tool uses a `system.md` file located in the project directory, which contains the system-level instructions for generating documentation.

### Using a Custom System Prompt

You can override the default `system.md` file by specifying your own system prompt file using the `--system-prompt` argument. This is useful if you need to tailor the instructions or formatting for different types of API documentation.

```bash
./target/release/auto-spec-ai --config routes.yaml --system-prompt custom_system.md
```

### System Prompt File Format

The system prompt file should be a simple text file containing the instructions for the model to follow when generating the documentation. For example, the system prompt might include information about how to format the OpenAPI schema, how to handle specific routes, or how to incorporate any extra metadata.

### Default System Prompt

If no `--system-prompt` argument is provided, Auto-Spec-AI will use the default `system.md` file included in the project directory. This file contains general instructions for generating OpenAPI documentation.

By customizing the system prompt, you can fine-tune how Auto-Spec-AI generates API documentation, making the tool more adaptable to your specific project needs.

#### The default `system.md` content:

```txt
You are an incredible assistant that converts the file content that are provided to an OpenAPI Specification 3.1.0 compatible configuration json. You will receive API route and controller files with other details for the route. Your task is to analyze each file content and generate the corresponding OpenAPI Specification 3.1.0 compatible configuration for that specific API endpoint only. This configuration will later be stored in a database and combined into a full API specification. Your response should be a **pure JSON response** that can be parsed programmatically. So be carefull not to include anything rather than the json object.

For each API endpoint, extract and generate the following information:

1. **URL and paths**: Identify the endpoint URL.
2. **HTTP method**: Detect the method used (GET, POST, PUT, DELETE, etc.).
3. **Parameters**: Extract any query parameters, path parameters, and payloads (body content).
4. **Request and response structure**: Document the expected request payload, headers, and response format.
5. **Permissions**: Extract any required permissions or roles needed to access the endpoint. Include these permissions in the `security` or `x-permissions` fields.
6. **Error handling**: Include relevant HTTP status codes and error messages, particularly related to permission issues (e.g., 403 Forbidden).

Your output should:

- Be in valid **JSON format** adhering to OpenAPI Specification 3.1.0.
- Contain only the configuration for the specific endpoint provided.
- Include no extra explanations, comments, or non-JSON text.

Ensure that:

- The output includes the necessary information under `paths` for that single endpoint.
- `components` are only included if relevant to that specific endpoint (e.g., schemas or parameters).
- The `security` or `x-permissions` fields reflect the permission requirements for accessing the endpoint.
- the output should not contain anything rather that the Open API json output. No explanations. No warnings. No extra notes.
- The output must include no extra explanations, comments, or non-JSON text.
- There is no explanations such as "Here is the OpenAPI Specification 3.1.0 configuration for this API endpoint:"

The configuration should be output as a **pure JSON response** without any additional information The output may be subject to programmatical parsing. So anything rather than **pure JSON output** may cause errors.
```

## Examples

Here are some common examples of how to use Auto-Spec-AI in different scenarios.

### 1. Using a YAML Configuration File

To generate API documentation for all routes defined in a configuration file, use the `--config` argument to specify the path to the YAML file:

```bash
./target/release/auto-spec-ai --config routes.yaml
```

The YAML file should define the routes, methods, and controllers for your API.

### 2. Generating Documentation for a Single Route

If you want to generate documentation for a single route, you can use the --route, --method, and --controller arguments directly without a configuration file:

```bash
./target/release/auto-spec-ai --route "/api/users" --method get --controller controllers/users_get.rs
```

### 3. Using Multiple Controllers for a Single Route

You can provide multiple controller files for a single route by separating them with commas:

```bash
./target/release/auto-spec-ai --route "/api/users" --method get --controller controllers/users_get.rs,controllers/extra_logic.rs
```

## Contributing

This is my first project built using Rust, and I am open to any kind of contribution! Whether you're experienced with Rust or just starting out, your input and improvements are highly appreciated.

Here are some ways you can contribute:

- **More Integrations**: While the project currently supports the Ollama API for LLM-based documentation generation, contributions to support additional integrations or APIs are welcome.
- **New Models**: At the moment, only the `llama3.1:8b` model is tested and supported. Contributions to add support for other models or help improve the system for new ones would be fantastic.
- **Code Improvements**: As this is my first Rust project, there’s definitely room for making the code cleaner, more efficient, or better structured. Feel free to submit improvements on this front.
- **New Arguments & Options**: If you have ideas for new command-line arguments or options that would enhance the tool’s functionality, I encourage you to submit them.

Feel free to open issues or submit pull requests on the GitHub repository if you'd like to contribute. Any help in making Auto-Spec-AI more powerful and efficient is greatly appreciated!

## License

This project is licensed under the Apache License 2.0. You are free to use, modify, and distribute this project, provided that you comply with the terms of the Apache 2.0 License.

You can read the full license text [here](https://www.apache.org/licenses/LICENSE-2.0).

## Contact

If you encounter any bugs, issues, or have suggestions, please feel free to use the **Issues** tab on the GitHub repository to report them. This is the best way to track bugs and feature requests, and it helps keep everything organized.

For private conversations or inquiries, you can reach me via email at:

peshraw@live.com

I look forward to hearing from you!

# Ragent - Rust AI Agent Framework

Ragent is a Rust-based framework designed to facilitate the creation of AI agents capable of interacting with various Large Language Model (LLM) providers and executing tools. This framework is built with extensibility and type safety in mind, providing a robust foundation for AI-driven applications.

## Features

- **Multi-provider LLM support**:
  - OpenAI (GPT-4)
  - Anthropic (Claude)
- **Extensible tool system**: Easily add and manage tools.
- **Async/await support**: Leverage Rust's async capabilities for efficient I/O operations.
- **Type-safe tool execution**: Ensure safety and correctness in tool operations.
- **Built-in error handling**: Robust error management for reliable applications.

## Project Structure

- **`ragent/`** - Core SDK library
  - `src/agent.rs` - Agent implementation
  - `src/client.rs` - LLM client traits
  - `src/clients/` - LLM provider implementations
  - `src/tool.rs` - Tool system traits
  - `src/tools/` - Built-in tools
- **`ragent_test/`** - Example application demonstrating the usage of the SDK

## Prerequisites

- Rust 2021 edition or later
- API keys for supported LLM providers:
  - OpenAI API key
  - Anthropic API key

## Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/ragent.git
   cd ragent
   ```

2. **Set up environment variables**:
   Create a `.env` file in the `ragent_test` directory with your API keys:
   ```env
   OPENAI_API_KEY=your_openai_api_key
   ANTHROPIC_API_KEY=your_anthropic_api_key
   ```

3. **Build the project**:
   Navigate to the `ragent_test` directory and build the example application:
   ```bash
   cd ragent_test
   cargo build
   ```

4. **Run the example application**:
   Execute the built application to see the framework in action:
   ```bash
   cargo run
   ```

## Functionality

- **Agent Management**: Create and manage AI agents with customizable properties such as name, description, and system prompts.
- **LLM Integration**: Seamlessly interact with multiple LLM providers using a unified client interface.
- **Tool Execution**: Define and execute tools with type-safe arguments and outputs, ensuring reliable operations.
- **Example Application**: The `ragent_test` application demonstrates how to use the SDK to perform tasks such as querying LLMs and executing tools.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details. 

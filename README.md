# test-claw-rust

A Rust-based AI agent that can interact with the file system and execute shell commands. This project serves as a starting point for building agents with basic tools.

## Features

- **Tool-use Capabilities**: Includes built-in tools for reading/writing files, searching for files, and executing bash commands.
- **Async Runtime**: Powered by `tokio` for efficient asynchronous execution.
- **OpenAI-Compatible API**: Uses `async-openai` and is pre-configured to work with OpenRouter.
- **Consent Mechanism**: Asks for user confirmation before executing potentially dangerous tools (can be bypassed with a flag).

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.95 or later recommended)
- An API key for OpenRouter (or another OpenAI-compatible service)

## Installation

Clone the repository and build the project:

```bash
cargo build --release
```

## Configuration

The agent is configured using environment variables. You can create a `.env` file in the project root:

```env
OPENROUTER_API_KEY=your_api_key_here
OPENROUTER_BASE_URL=https://openrouter.ai/api/v1
LOCAL_TEST_MODEL=anthropic/claude-3.5-sonnet:beta
```

- `OPENROUTER_API_KEY`: Your API key.
- `OPENROUTER_BASE_URL`: Base URL for the API (defaults to OpenRouter).
- `LOCAL_TEST_MODEL`: The model ID to use (defaults to `anthropic/claude-haiku-4.5`).

## Usage

Run the agent with a prompt using the `--prompt` (or `-p`) flag:

```bash
cargo run -- -p "List all files in the current directory and read the Cargo.toml file."
```

### Options

- `-p`, `--prompt <PROMPT>`: The initial prompt for the agent.
- `-y`, `--yes`: Automatically say yes to all tool execution requests (skip manual consent).

## Available Tools

The agent currently supports the following tools:

- **Read**: Reads the content of a file.
- **Write**: Writes content to a file.
- **Glob**: Searches for files using glob patterns.
- **Bash**: Executes shell commands.

## Project Structure

- `src/main.rs`: Entry point and CLI argument parsing.
- `src/agent/`: Core agent loop and conversation management.
- `src/tools/`: Implementation of tools and tool dispatch logic.
- `src/ui/`: UI components (spinner, consent prompts).
- `src/error.rs`: Centralized error handling.

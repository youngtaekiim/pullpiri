# SettingsCLI

A command-line interface tool for interacting with the Pullpiri SettingsService via REST APIs.

## Overview

SettingsCLI provides developers with a convenient way to access all REST APIs provided by the SettingsService. Since SettingsService runs as a container, this separated CLI app can be used anywhere on the system where developers need to interact with the service.

## Features

- **Metrics Operations**: Get system metrics and board information
- **Board Management**: List and inspect board configurations
- **Node Management**: Monitor and manage individual nodes
- **SoC Management**: Handle System-on-Chip resource management
- **Health Checks**: Verify SettingsService connectivity
- **Flexible Output**: Support for both formatted and raw JSON output
- **Comprehensive Help**: Built-in help system for all commands

## Installation

Build the CLI tool as part of the Pullpiri project:

```bash
cd src/tools/settingscli
cargo build --release
```

Or build all tools together:

```bash
cd src/tools
cargo build --release
```

## Usage

### Basic Syntax

```bash
settingscli [OPTIONS] <COMMAND>
```

### Global Options

- `-u, --url <URL>`: SettingsService URL (default: http://localhost:47098)
- `-t, --timeout <SECONDS>`: Request timeout in seconds (default: 30)
- `-v, --verbose`: Enable verbose output
- `-h, --help`: Print help information
- `-V, --version`: Print version information

### Commands

#### Health Check

Test connection to SettingsService:

```bash
settingscli health
```

#### Metrics

Get system metrics:

```bash
# Get formatted metrics
settingscli metrics get

# Get raw JSON metrics
settingscli metrics raw
```

#### Board Operations

```bash
# List all boards
settingscli board list

# Get specific board information
settingscli board get <BOARD_ID>

# Get raw board data
settingscli board raw [BOARD_ID]
```

#### Node Operations

```bash
# List all nodes
settingscli node list

# Get specific node information
settingscli node get <NODE_ID>

# Get raw node data
settingscli node raw [NODE_ID]
```

#### SoC Operations

```bash
# List all SoCs
settingscli soc list

# Get specific SoC information
settingscli soc get <SOC_ID>

# Get raw SoC data
settingscli soc raw [SOC_ID]
```

### Examples

```bash
# Check if SettingsService is running
settingscli health

# Get system metrics with custom URL and timeout
settingscli -u http://192.168.1.100:47098 -t 60 metrics get

# List all boards with verbose output
settingscli -v board list

# Get specific node details
settingscli node get HPC

# Get raw JSON output for a specific SoC
settingscli soc raw 192.168.225.30
```

## Testing

### Prerequisites

Before running integration tests, ensure the following services are running:

1. **Start podman.socket service:**
   ```bash
   sudo systemctl start podman.socket
   ```

2. **Start required services in order:**
   ```bash
   # 1. Start statemanager
   cargo run --manifest-path=src/player/statemanager/Cargo.toml

   # 2. Start monitoringserver
   cargo run --manifest-path=src/server/monitoringserver/Cargo.toml

   # 3. Start settingsservice
   cargo run --manifest-path=src/server/settingsservice/Cargo.toml

   # 4. Start nodeagent
   cargo run --manifest-path=src/agent/nodeagent/Cargo.toml
   ```

### Running Tests

```bash
# Run unit tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_client_creation
```

### Integration Testing

With all services running, test the CLI:

```bash
# Test basic connectivity
./target/release/settingscli health

# Test metrics endpoint
./target/release/settingscli metrics get

# Test all endpoints
./target/release/settingscli board list
./target/release/settingscli node list
./target/release/settingscli soc list
```

## Error Handling

The CLI provides clear error messages for common issues:

- **Connection errors**: When SettingsService is unreachable
- **Timeout errors**: When requests take too long
- **JSON parsing errors**: When response format is unexpected
- **HTTP errors**: When API endpoints return error status codes

## Development

### Project Structure

```
src/tools/settingscli/
├── Cargo.toml          # Dependencies and metadata
├── src/
│   ├── main.rs         # CLI entry point
│   ├── lib.rs          # Library exports
│   ├── client.rs       # HTTP client implementation
│   ├── error.rs        # Error handling
│   └── commands/       # Command implementations
│       ├── mod.rs      # Command utilities
│       ├── metrics.rs  # Metrics operations
│       ├── board.rs    # Board operations
│       ├── node.rs     # Node operations
│       └── soc.rs      # SoC operations
└── tests/              # Integration tests
    ├── integration_test.rs
    └── cli_test.rs
```

### Adding New Commands

1. Create a new file in `src/commands/`
2. Implement the command logic with appropriate error handling
3. Add the command to the main CLI enum in `main.rs`
4. Update the command dispatcher
5. Add tests for the new functionality

### Code Validation

Before committing changes, always run:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
scripts/fmt_check.sh
scripts/clippy_check.sh
make build
```

**NEVER CANCEL** these validation steps.

## Related Components

- **SettingsService**: The REST API server this CLI communicates with
- **Pullpiri Core**: The main vehicle service orchestrator framework
- **Monitoring Server**: Provides system metrics and monitoring data
- **State Manager**: Manages application state and configuration

## License

This project is licensed under the Apache-2.0 license.
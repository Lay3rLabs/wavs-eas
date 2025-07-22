# [WAVS](https://docs.wavs.xyz) EAS Template

**Template for getting started with developing WAVS applications**

A Ethereum Attestation Service template for developing Web Assembly Verifiable Services using Rust and Solidity, configured for Windows _WSL_, Linux, and MacOS.

<!-- **Languages**
 * [Rust (this example)](./components/evm-price-oracle/)
 * [JS / TS](./components/js-evm-price-oracle/README.md) -->

## System Requirements

<details>
<summary>Core (Docker, Compose, Make, JQ, Node v21+, Foundry)</summary>

## Ubuntu Base

- **Linux**: `sudo apt update && sudo apt install build-essential`

### Docker

If prompted, remove container with `sudo apt remove containerd.io`.

- **MacOS**: `brew install --cask docker`
- **Linux**: `sudo apt -y install docker.io`
- **Windows WSL**: [docker desktop wsl](https://docs.docker.com/desktop/wsl/#turn-on-docker-desktop-wsl-2) & `sudo chmod 666 /var/run/docker.sock`
- [Docker Documentation](https://docs.docker.com/get-started/get-docker/)

> **Note:** `sudo` is only used for Docker-related commands in this project. If you prefer not to use sudo with Docker, you can add your user to the Docker group with:
>
> ```bash
> sudo groupadd docker && sudo usermod -aG docker $USER
> ```
>
> After adding yourself to the group, log out and back in for changes to take effect.

### Docker Compose

- **MacOS**: Already installed with Docker installer
  > `sudo apt remove docker-compose-plugin` may be required if you get a `dpkg` error
- **Linux + Windows WSL**: `sudo apt-get install docker-compose-v2`
- [Compose Documentation](https://docs.docker.com/compose/)

### Make

- **MacOS**: `brew install make`
- **Linux + Windows WSL**: `sudo apt -y install make`
- [Make Documentation](https://www.gnu.org/software/make/manual/make.html)

### JQ

- **MacOS**: `brew install jq`
- **Linux + Windows WSL**: `sudo apt -y install jq`
- [JQ Documentation](https://jqlang.org/download/)

### Node.js

- **Required Version**: v21+
- [Installation via NVM](https://github.com/nvm-sh/nvm?tab=readme-ov-file#installing-and-updating)

```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
nvm install --lts
```

### Foundry

```bash docci-ignore
curl -L https://foundry.paradigm.xyz | bash && $HOME/.foundry/bin/foundryup
```

</details>

<details>

<summary>Rust v1.85+</summary>

### Rust Installation

```bash docci-ignore
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustup toolchain install stable
rustup target add wasm32-wasip2
```

### Upgrade Rust

```bash docci-ignore
# Remove old targets if present
rustup target remove wasm32-wasi || true
rustup target remove wasm32-wasip1 || true

# Update and add required target
rustup update stable
rustup target add wasm32-wasip2
```

</details>

<details>
<summary>Cargo Components</summary>

### Install Cargo Components

On Ubuntu LTS, if you later encounter errors like:

```bash
wkg: /lib/x86_64-linux-gnu/libm.so.6: version `GLIBC_2.38' not found (required by wkg)
wkg: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.39' not found (required by wkg)
```

If GLIB is out of date. Consider updating your system using:

```bash
sudo do-release-upgrade
```

```bash docci-ignore
# Install required cargo components
# https://github.com/bytecodealliance/cargo-component#installation
cargo install cargo-binstall
cargo binstall cargo-component wasm-tools warg-cli wkg --locked --no-confirm --force

# Configure default registry
# Found at: $HOME/.config/wasm-pkg/config.toml
wkg config --default-registry wa.dev

# Allow publishing to a registry
#
# if WSL: `warg config --keyring-backend linux-keyutils`
warg key new
```

</details>

## Create Project

```bash docci-ignore
# if foundry is not installed:
# `curl -L https://foundry.paradigm.xyz | bash && $HOME/.foundry/bin/foundryup`
forge init --template Lay3rLabs/wavs-eas my-wavs-eas-app --branch main
```

> \[!TIP]
> Run `make help` to see all available commands and environment variable overrides.

### Solidity

Install the required packages to build the Solidity contracts. This project supports both [submodules](./.gitmodules) and [npm packages](./package.json).

```bash
# Install packages (npm & submodules)
make setup

# Build the contracts
forge build

# Run the solidity tests
forge test
```

## Build WASI components

Now build the WASI components into the `compiled` output directory.

> \[!WARNING]
> If you get: `error: no registry configured for namespace "wavs"`
>
> run, `wkg config --default-registry wa.dev`

> \[!WARNING]
> If you get: `failed to find the 'wasm32-wasip1' target and 'rustup' is not available`
>
> `brew uninstall rust` & install it from <https://rustup.rs>

```bash
make wasi-build
```

## Testing the Price Feed Component Locally

How to test the component locally for business logic validation before on-chain deployment.

TODO! Update this to actually work with our components

```bash
make wasi-exec
```

Expected output:

```shell docci-ignore

```

## WAVS

> \[!NOTE]
> If you are running on a Mac with an ARM chip, you will need to do the following:
>
> - Set up Rosetta: `softwareupdate --install-rosetta`
> - Enable Rosetta (Docker Desktop: Settings -> General -> enable "Use Rosetta for x86_64/amd64 emulation on Apple Silicon")
>
> Configure one of the following networking:
>
> - Docker Desktop: Settings -> Resources -> Network -> 'Enable Host Networking'
> - `brew install chipmk/tap/docker-mac-net-connect && sudo brew services start chipmk/tap/docker-mac-net-connect`

## Start Environment

Start an ethereum node (anvil), the WAVS service, and deploy [eigenlayer](https://www.eigenlayer.xyz/) contracts to the local network.

### Enable Telemetry (optional)

Set Log Level:

- Open the `.env` file.
- Set the `log_level` variable for wavs to debug to ensure detailed logs are captured.

> \[!NOTE]
> To see details on how to access both traces and metrics, please check out [Telemetry Documentation](telemetry/telemetry.md).

### Start the backend

```bash docci-background docci-delay-after=5
# This must remain running in your terminal. Use another terminal to run other commands.
# You can stop the services with `ctrl+c`. Some MacOS terminals require pressing it twice.
cp .env.example .env

# update the .env for either LOCAL or TESTNET

# Starts anvil + IPFS, WARG, Jaeger, and prometheus.
make start-all-local
```

## WAVS Deployment Script

This script automates the complete WAVS deployment process in a single command:

### What It Does

1. **Build Check**: Rebuilds WebAssembly component if changes detected
2. **Create Deployer**: Sets up and funds deployer account
3. **Deploy Eigenlayer**: Deploys service manager contract
4. **Deploy Contracts**: Creates trigger and submission contracts
5. **Upload Component**: Publishes WebAssembly component to WASI registry
6. **Build Service**: Creates service configuration
7. **Upload to IPFS**: Stores service metadata on IPFS
8. **Set Service URI**: Registers IPFS URI with service manager
9. **Start Aggregator**: Launches result aggregation service
10. **Start WAVS**: Launches operator service with readiness check
11. **Deploy Service**: Configures WAVS to monitor trigger events
12. **Generate Keys**: Creates operator signing keys
13. **Register Operator**: Registers with Eigenlayer AVS (0.001 ETH stake)
14. **Verify Registration**: Confirms operator registration

**Result:** A fully operational WAVS service that monitors blockchain events, executes WebAssembly components, and submits verified results on-chain.

```bash
export RPC_URL=`bash ./script/get-rpc.sh`
export AGGREGATOR_URL=http://localhost:8001

bash ./script/deploy-script.sh
```

# EAS Attestation Trigger Scripts

Scripts for triggering EAS attestation creation via WAVS components.

## Available Scripts

### Trigger.s.sol

**Purpose**: Comprehensive EAS attestation trigger script with clean API for testing and production use.

**Core Trigger Functions**:

- `triggerEASAttestation(serviceTriggerAddr, data)` - Creates EAS attestation with default schema
- `triggerCustomEASAttestation(serviceTriggerAddr, schema, recipient, data)` - Creates custom EAS attestation
- `triggerJsonAttestation(triggerAddr, schema, recipient, data)` - Creates structured JSON attestation
- `triggerRawAttestation(triggerAddr, rawData)` - Creates raw data attestation

**Example Functions**:

- `triggerTestimonialExample(triggerAddr)` - Example testimonial attestation
- `triggerSkillVerificationExample(triggerAddr, skillHolder)` - Example skill verification

**Query Functions**:

- `showTrigger(serviceTriggerAddr)` - Shows current trigger ID
- `queryAttestations(easAddr, schemaId, recipient)` - Query EAS attestations by schema/recipient
- `showAttestation(easAddr, attestationUid)` - Show detailed attestation information

## Environment Setup

Set these variables from deployment output:

```bash
# Parse deployment addresses from .docker/deployment_summary.json
export RPC_URL=`bash ./script/get-rpc.sh`
export SERVICE_TRIGGER_ADDR=$(jq -r '.service_contracts.trigger' .docker/deployment_summary.json)
export EAS_ADDR=$(jq -r '.eas_contracts.eas' .docker/deployment_summary.json)
```

## Usage Examples

### Basic Attestation (Using Default Schema)

Create a basic EAS Attestation:

```bash
# Simple attestation with default values
forge script script/Trigger.s.sol:EasTrigger --sig "triggerEASAttestation(string,string)" \
  "${SERVICE_TRIGGER_ADDR}" \
  "This person completed the Solidity course" \
  --rpc-url $RPC_URL --broadcast
```

### Custom JSON Attestation

Create a custom EAS Attestation:

```bash
# Custom attestation with specific schema and recipient
forge script script/Trigger.s.sol:EasTrigger --sig "triggerJsonAttestation(string,string,string,string)" \
  "${SERVICE_TRIGGER_ADDR}" \
  "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef" \
  "0x742d35Cc6634C0532925a3b8D4f3e9dC9BfD16BB" \
  "Advanced Solidity Development Skills Verified" \
  --rpc-url $RPC_URL --broadcast
```

### Raw Data Attestation

Request creation of an Attestation via the WAVS service:

```bash
# Raw attestation data (uses component defaults)
forge script script/Trigger.s.sol:EasTrigger --sig "triggerRawAttestation(string,string)" \
  "${SERVICE_TRIGGER_ADDR}" \
  "Simple testimonial: Trustworthy individual" \
  --rpc-url $RPC_URL --broadcast
```

### Example Use Cases

Request creation of an Attestation via the WAVS service:

```bash
# Create a testimonial attestation
forge script script/Trigger.s.sol:EasTrigger --sig "triggerTestimonialExample(string)" \
  "${SERVICE_TRIGGER_ADDR}" \
  --rpc-url $RPC_URL --broadcast

# Create a skill verification for a specific address
forge script script/Trigger.s.sol:EasTrigger --sig "triggerSkillVerificationExample(string,string)" \
  "${SERVICE_TRIGGER_ADDR}" \
  "0x742d35Cc6634C0532925a3b8D4f3e9dC9BfD16BB" \
  --rpc-url $RPC_URL --broadcast
```

### Query Functions

```bash
# Show current trigger state
forge script script/Trigger.s.sol:EasTrigger --sig "showTrigger(string)" \
  "${SERVICE_TRIGGER_ADDR}" \
  --rpc-url $RPC_URL

# Show specific attestation details
forge script script/Trigger.s.sol:EasTrigger --sig "showAttestation(string,string)" \
  "${EAS_ADDR}" \
  "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef" \
  --rpc-url $RPC_URL

# Query attestations by schema and recipient
forge script script/Trigger.s.sol:EasTrigger --sig "queryAttestations(string,string,string)" \
  "${EAS_ADDR}" \
  "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef" \
  "0x742d35Cc6634C0532925a3b8D4f3e9dC9BfD16BB" \
  --rpc-url $RPC_URL
```

## AI Coding Agents

This template contains rulefiles for building components with Claude Code and Cursor. Read the [AI-powered component creation guide](./docs/handbook/ai.mdx) for usage instructions.

### Claude Code

To spin up a sandboxed instance of [Claude Code](https://docs.anthropic.com/en/docs/agents-and-tools/claude-code/overview) in a Docker container that only has access to this project's files, run the following command:

```bash docci-ignore
npm run claude-code
# or with no restrictions (--dangerously-skip-permissions)
npm run claude-code:unrestricted
```

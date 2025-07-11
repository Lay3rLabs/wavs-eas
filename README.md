# [WAVS](https://docs.wavs.xyz) EAS Template

**Template for getting started with developing WAVS applications**

A Ethereum Attestation Service template for developing Web Assembly Verifiable Services using Rust and Solidity, configured for Windows *WSL*, Linux, and MacOS.

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
> ```bash
> sudo groupadd docker && sudo usermod -aG docker $USER
> ```
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
> - Set up Rosetta: `softwareupdate --install-rosetta`
> - Enable Rosetta (Docker Desktop: Settings -> General -> enable "Use Rosetta for x86_64/amd64 emulation on Apple Silicon")
>
> Configure one of the following networking:
> - Docker Desktop: Settings -> Resources -> Network -> 'Enable Host Networking'
> - `brew install chipmk/tap/docker-mac-net-connect && sudo brew services start chipmk/tap/docker-mac-net-connect`

## Start Environment

Start an ethereum node (anvil), the WAVS service, and deploy [eigenlayer](https://www.eigenlayer.xyz/) contracts to the local network.

### Enable Telemetry (optional)

Set Log Level:
  - Open the `.env` file.
  - Set the `log_level` variable for wavs to debug to ensure detailed logs are captured.

> \[!NOTE]
To see details on how to access both traces and metrics, please check out [Telemetry Documentation](telemetry/telemetry.md).

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

### Result

A fully operational WAVS service that monitors blockchain events, executes WebAssembly components, and submits verified results on-chain.

```bash
export RPC_URL=`bash ./script/get-rpc.sh`
export AGGREGATOR_URL=http://127.0.0.1:8001

bash ./script/deploy-script.sh
```

## Create Deployer, upload Eigenlayer

These sections can be run on the **same** machine, or separate for testnet environments. Run the following steps on the deployer/aggregator machine.

```bash
# local: create deployer & auto fund. testnet: create & iterate check balance
bash ./script/create-deployer.sh

## Deploy Eigenlayer from Deployer
COMMAND=deploy make wavs-middleware
```

## Deploy Service Contracts

`WAVS_SERVICE_MANAGER_ADDRESS` is the address of the Eigenlayer service manager contract. It was deployed in the previous step. Then you deploy the trigger and submission contracts which depends on the service manager. The service manager will verify that a submission is valid (from an authorized operator) before saving it to the blockchain. The trigger contract is any arbitrary contract that emits some event that WAVS will watch for. Yes, this can be on another chain (e.g. an L2) and then the submission contract on the L1 *(Ethereum for now because that is where Eigenlayer is deployed)*.

```bash docci-delay-per-cmd=2
source script/deploy-contracts.sh
```

## Deploy Service

Deploy the compiled component with the contract information from the previous steps. Review the [makefile](./Makefile) for more details and configuration options.`TRIGGER_EVENT` is the event that the trigger contract emits and WAVS watches for. By altering `SERVICE_TRIGGER_ADDR` you can watch events for contracts others have deployed.

```bash docci-delay-per-cmd=3
export COMPONENT_FILENAME=wavs_eas_attest.wasm
export PKG_NAME="easattest"
export PKG_VERSION="0.1.0"
# ** Testnet Setup: https://wa.dev/account/credentials/new -> warg login
source script/upload-to-wasi-registry.sh || true

# Testnet: set values (default: local if not set)
# export TRIGGER_CHAIN=holesky
# export SUBMIT_CHAIN=holesky

# Package not found with wa.dev? -- make sure it is public
export AGGREGATOR_URL=http://127.0.0.1:8001
REGISTRY=${REGISTRY} source ./script/build-service.sh
```

## Upload to IPFS

```bash
# Upload service.json to IPFS
SERVICE_FILE=.docker/service.json source ./script/ipfs-upload.sh
```

## Start Aggregator

**TESTNET** You can move the aggregator it to its own machine for testnet deployments, it's easiest to run this on the deployer machine first. If moved, ensure you set the env variables correctly (copied and pasted from the previous steps on the other machine).

```bash
bash ./script/create-aggregator.sh 1

IPFS_GATEWAY=${IPFS_GATEWAY} bash ./infra/aggregator-1/start.sh

wget -q --header="Content-Type: application/json" --post-data="{\"uri\": \"${IPFS_URI}\"}" ${AGGREGATOR_URL}/register-service -O -
```

## Start WAVS

**TESTNET** The WAVS service should be run in its own machine (creation, start, and opt-in). If moved, make sure you set the env variables correctly (copy pasted from the previous steps on the other machine).

```bash
bash ./script/create-operator.sh 1

IPFS_GATEWAY=${IPFS_GATEWAY} bash ./infra/wavs-1/start.sh

# Deploy the service JSON to WAVS so it now watches and submits.
# 'opt in' for WAVS to watch (this is before we register to Eigenlayer)
WAVS_ENDPOINT=http://127.0.0.1:8000 SERVICE_URL=${IPFS_URI} IPFS_GATEWAY=${IPFS_GATEWAY} make deploy-service
```

## Register service specific operator

Making test mnemonic: `cast wallet new-mnemonic --json | jq -r .mnemonic`

Each service gets their own key path (hd_path). The first service starts at 1 and increments from there. Get the service ID

```bash
SERVICE_INDEX=0 source ./script/avs-signing-key.sh

# TESTNET: set WAVS_SERVICE_MANAGER_ADDRESS
export WAVS_SERVICE_MANAGER_ADDRESS=$(jq -r .addresses.WavsServiceManager ./.nodes/avs_deploy.json)
COMMAND="register ${OPERATOR_PRIVATE_KEY} ${AVS_SIGNING_ADDRESS} 0.001ether" make wavs-middleware

# Verify registration
COMMAND="list_operator" PAST_BLOCKS=500 make wavs-middleware
```

## Trigger the Service

Anyone can now call the [trigger contract](./src/contracts/WavsTrigger.sol) which emits the trigger event WAVS is watching for from the previous step. WAVS then calls the service and saves the result on-chain.

NOTE: the trigger contract is for example purposes only.

```bash
# Rust & Typescript - request BTC from CMC
export INPUT_DATA=`cast abi-encode "addTrigger(string)" "1"`

# Golang uses the raw value
# export INPUT_DATA="1"

# Get the trigger address from previous Deploy forge script
export SERVICE_TRIGGER_ADDR=`make get-trigger-from-deploy`
# Execute on the trigger contract, WAVS will pick this up and submit the result
# on chain via the operators.

# uses FUNDED_KEY as the executor (local: anvil account)
source .env
export RPC_URL=`sh ./script/get-rpc.sh`

forge script ./script/Trigger.s.sol ${SERVICE_TRIGGER_ADDR} ${INPUT_DATA} --sig 'run(string,string)' --rpc-url ${RPC_URL} --broadcast
```

## Show the result

Query the latest submission contract id from the previous request made.

```bash
forge script ./script/ShowResult.s.sol ${SERVICE_TRIGGER_ADDR} --sig 'trigger(string)' --rpc-url $(RPC_URL) --broadcast
```

```bash
forge script ./script/ShowResult.s.sol ${SERVICE_SUBMISSION_ADDR} ${TRIGGER_ID} --sig 'data(string,uint64)' --rpc-url $(RPC_URL) --broadcast
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

#!/bin/bash

# Deploy EAS contracts and WAVS EAS integration
# This script replaces the old deploy-contracts.sh with EAS-based deployment

set -e

echo "🚀 Starting WAVS EAS contract deployment..."

# Check for required WAVS service manager address
if [ -z "$WAVS_SERVICE_MANAGER_ADDRESS" ]; then
    if [ -f .nodes/avs_deploy.json ]; then
        echo "📋 Using WAVS_SERVICE_MANAGER_ADDRESS from .nodes/avs_deploy.json"
        export WAVS_SERVICE_MANAGER_ADDRESS=$(jq -r '.addresses.WavsServiceManager' .nodes/avs_deploy.json)
    else
        echo "❌ WAVS_SERVICE_MANAGER_ADDRESS is not set and .nodes/avs_deploy.json not found."
        echo "   Please set WAVS_SERVICE_MANAGER_ADDRESS or ensure AVS deployment is complete."
        exit 1
    fi
fi

# Get RPC URL and deployer key
export RPC_URL=$(bash ./script/get-rpc.sh)
export DEPLOYER_PK=$(cat .nodes/deployer)

echo "🔧 Configuration:"
echo "   RPC_URL: ${RPC_URL}"
echo "   WAVS_SERVICE_MANAGER_ADDRESS: ${WAVS_SERVICE_MANAGER_ADDRESS}"

# Create output directory
mkdir -p .docker

echo "📦 Deploying EAS contracts..."

# Deploy EAS contracts using Foundry script
forge script script/DeployEAS.s.sol:DeployEAS \
    --sig 'run(string)' "${WAVS_SERVICE_MANAGER_ADDRESS}" \
    --rpc-url "${RPC_URL}" \
    --private-key "${DEPLOYER_PK}" \
    --broadcast \
    --json > .docker/eas_deploy.json

# Extract deployed addresses
export EAS_REGISTRY_ADDR=$(jq -r '.logs[] | select(type == "string" and startswith("SchemaRegistry deployed at:")) | split(": ")[1]' .docker/eas_deploy.json 2>/dev/null || echo "")
export EAS_ADDR=$(jq -r '.logs[] | select(type == "string" and startswith("EAS deployed at:")) | split(": ")[1]' .docker/eas_deploy.json 2>/dev/null || echo "")
export EAS_ATTESTER_ADDR=$(jq -r '.logs[] | select(type == "string" and startswith("Attester deployed at:")) | split(": ")[1]' .docker/eas_deploy.json 2>/dev/null || echo "")
export EAS_SCHEMA_REGISTRAR_ADDR=$(jq -r '.logs[] | select(type == "string" and startswith("SchemaRegistrar deployed at:")) | split(": ")[1]' .docker/eas_deploy.json 2>/dev/null || echo "")
export EAS_LOG_RESOLVER_ADDR=$(jq -r '.logs[] | select(type == "string" and startswith("LogResolver deployed at:")) | split(": ")[1]' .docker/eas_deploy.json 2>/dev/null || echo "")

# Still deploy SimpleTrigger as it's used for triggering events
echo "📦 Deploying SimpleTrigger..."
forge create SimpleTrigger \
    --json \
    --broadcast \
    --rpc-url "${RPC_URL}" \
    --private-key "${DEPLOYER_PK}" > .docker/trigger.json

export SERVICE_TRIGGER_ADDR=$(jq -r '.deployedTo' .docker/trigger.json)

# Create consolidated deployment info
cat > .docker/deployment_summary.json << EOF
{
  "rpc_url": "${RPC_URL}",
  "wavs_service_manager": "${WAVS_SERVICE_MANAGER_ADDRESS}",
  "eas_contracts": {
    "schema_registry": "${EAS_REGISTRY_ADDR}",
    "eas": "${EAS_ADDR}",
    "attester": "${EAS_ATTESTER_ADDR}",
    "schema_registrar": "${EAS_SCHEMA_REGISTRAR_ADDR}",
    "log_resolver": "${EAS_LOG_RESOLVER_ADDR}"
  },
  "service_contracts": {
    "trigger": "${SERVICE_TRIGGER_ADDR}"
  }
}
EOF

echo "✅ EAS Deployment Complete!"
echo ""
echo "📋 Deployment Summary:"
echo "   RPC_URL: ${RPC_URL}"
echo "   WAVS_SERVICE_MANAGER_ADDRESS: ${WAVS_SERVICE_MANAGER_ADDRESS}"
echo ""
echo "🏗️  EAS Contracts:"
echo "   EAS_REGISTRY_ADDR: ${EAS_REGISTRY_ADDR}"
echo "   EAS_ADDR: ${EAS_ADDR}"
echo "   EAS_ATTESTER_ADDR: ${EAS_ATTESTER_ADDR}"
echo "   EAS_SCHEMA_REGISTRAR_ADDR: ${EAS_SCHEMA_REGISTRAR_ADDR}"
echo "   EAS_LOG_RESOLVER_ADDR: ${EAS_LOG_RESOLVER_ADDR}"
echo ""
echo "🎯 Service Contracts:"
echo "   SERVICE_TRIGGER_ADDR: ${SERVICE_TRIGGER_ADDR}"
echo ""
echo "📄 Deployment details saved to .docker/deployment_summary.json"
echo "📄 Full deployment logs saved to .docker/eas_deploy.json"

# Update environment variables for other scripts
export SERVICE_SUBMISSION_ADDR="${EAS_ATTESTER_ADDR}"  # For backwards compatibility

echo ""
echo "🔄 Environment Variables Set:"
echo "   SERVICE_SUBMISSION_ADDR=${SERVICE_SUBMISSION_ADDR} (Attester contract)"
echo "   SERVICE_TRIGGER_ADDR=${SERVICE_TRIGGER_ADDR}"

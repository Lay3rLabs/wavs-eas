#!/bin/bash

# Temporary fix for EAS deployment script when forge is not available
# This script manually sets the known deployed addresses

echo "🔧 Manual deployment fix - setting EAS addresses from successful deployment..."

# Set the known deployed addresses from the successful deployment
export EAS_REGISTRY_ADDR="0x613D57a889505F6ab1faE62eF91739F30CF559F6"
export EAS_ADDR="0x07dE2e64901b0A7eaB8F423404Ec2D6c29F829B4"
export EAS_ATTESTER_ADDR="0xd71C006a20cade2d003694B5E4436A2D7e0B5B1c"
export EAS_SCHEMA_REGISTRAR_ADDR="0xCdb685776c18616a34C346579244385453EA2B12"
export EAS_LOG_RESOLVER_ADDR="0xd16c08759881B4C84e2D0531AE54A7d8A7aCb36f"
export SERVICE_TRIGGER_ADDR="0x4134578201bf8fEF14A18d880d5fD15c52BAcBAC"

# Get configuration from existing files
export WAVS_SERVICE_MANAGER_ADDRESS=$(jq -r '.addresses.WavsServiceManager' .nodes/avs_deploy.json)
export RPC_URL="http://localhost:8545"

# Create consolidated deployment info
mkdir -p .docker
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

echo "✅ Manual deployment fix complete!"
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

# Update environment variables for other scripts
export SERVICE_SUBMISSION_ADDR="${EAS_ATTESTER_ADDR}"

echo ""
echo "🔄 Environment Variables Set:"
echo "   SERVICE_SUBMISSION_ADDR=${SERVICE_SUBMISSION_ADDR} (Attester contract)"
echo "   SERVICE_TRIGGER_ADDR=${SERVICE_TRIGGER_ADDR}"
echo ""
echo "💡 The contracts are already deployed. This script just sets the environment variables."
echo "   If you need to redeploy, ensure 'forge' is available in your PATH."
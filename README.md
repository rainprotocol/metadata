# rain.metadata

## Specs

MetadataV1 spec - https://github.com/rainprotocol/specs/blob/main/metadata-v1.md

# Test
export *DEPLOYER_KEY* and *RPC_URL* in env
## Start Docker container
> docker-compose -f subgraph/docker/docker-compose.yaml up -d

## Run Forge Script
> forge script scripts/EmitMeta.s.sol --ffi --broadcast --rpc-url $RPC_URL
> forge script scripts/DeploySubgrpah.s.sol --ffi --rpc-url $RPC_URL
> forge script scripts/MetaBoardEntityTest.s.sol --ffi --rpc-url $RPC_URL
> forge script scripts/MetaV1EntityTest.s.sol --ffi --rpc-url $RPC_URL
> forge script scripts/MultiEmitMeta.s.sol --ffi --broadcast --rpc-url $RPC_URL

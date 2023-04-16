steps
- nix-shell --run init
- nix-shell --run docker-up
- nix-shell --run local-test

to deploy the subgraph
- `nix-shell --run init`
- start nix-shell. type `nix-shell` terminal
- execute the following command in nix-shell
    `ts-node scripts/deploy.ts --contractAddress <CONTRACT_ADDRESS> --subgraphName <SUBGRAPH_NAME> --graphAccessToken <GRAPH_ACCESS_TOKEN> --network <NETWORK> --blockNumber <BLOCK_NUMBER>`

{
  description = "Flake for development metaboard subgraph workflows.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/ec750fd01963ab6b20ee1f0cb488754e8036d89d";
    flake-utils.url = "github:numtide/flake-utils";
  };


  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        # jq = "${pkgs.jq}/bin/jq";

      in rec {
        packages = rec {
          init =  pkgs.writeShellScriptBin "init" (''
            npm install
            mkdir -p contracts && cp ../src/* contracts

            ${compile}

            rm -rf ./abis && mkdir ./abis
            cp artifacts/contracts/concrete/MetaBoard.sol/MetaBoard.json abis
          '');

          ci-test = pkgs.writeShellScriptBin "ci-test" (''
            npx mustache config/localhost.json subgraph.template.yaml subgraph.yaml
            npx graph codegen
            npm run test
          '');

          compile = pkgs.writeShellScriptBin "compile" (''
            hardhat compile --force
          '');

          docker-up = pkgs.writeShellScriptBin "docker-up" ''
            docker-compose -f docker/docker-compose.yml up --build -d
          '';

          docker-down = pkgs.writeShellScriptBin "docker-down" ''
            docker-compose -f docker/docker-compose.yml down
          '';

          default = init;
        };
      }
    );
}

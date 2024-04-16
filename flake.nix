{
  description = "Flake for development workflows.";

  inputs = {
    rainix.url = "github:rainprotocol/rainix";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {self, flake-utils, rainix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = rainix.pkgs.${system};
        rust-toolchain = rainix.rust-toolchain.${system};
      in rec {
        packages = rec {
          mkBin = (pkgs.makeRustPlatform{
            rustc = rust-toolchain;
            cargo = rust-toolchain;
          }).buildRustPackage {
            src = ./.;
            doCheck = false;
            name = "rain-metadata";
            cargoLock.lockFile = ./Cargo.lock;
            # allows for git deps to be resolved without the need to specify their outputHash
            cargoLock.allowBuiltinFetchGit = true;
            buildPhase = ''
              cargo build --release --bin rain-metadata --all-features
            '';
            installPhase = ''
              mkdir -p $out/bin
              cp target/release/rain-metadata $out/bin/
            '';
            buildInputs = with pkgs; [
              openssl
            ];
            nativeBuildInputs = with pkgs; [
              pkg-config
            ] ++ lib.optionals stdenv.isDarwin [
              darwin.apple_sdk.frameworks.SystemConfiguration
            ];
          };

          subgraph-build = rainix.mkTask.${system} {
            name = "subgraph-build";
            body = ''
              set -euxo pipefail
              forge build
              cd ./subgraph;
              npm install;
              graph codegen;
              graph build --network matic;
              cd -;
            '';
          };

          subgraph-deploy = rainix.mkTask.${system} {
            name = "subgraph-deploy";
            body = ''
              set -euo pipefail
              ${subgraph-build}/bin/subgraph-build
              goldsky --token ''${GOLDSKY_TOKEN} subgraph deploy ''${GOLDSKY_NAME_AND_VERSION}
            '';
          };
        } // rainix.packages.${system};

        devShells.default = pkgs.mkShell {
          packages = [
            packages.subgraph-build
            packages.subgraph-deploy
          ];
          shellHook = rainix.devShells.${system}.default.shellHook;
          buildInputs = rainix.devShells.${system}.default.buildInputs;
          nativeBuildInputs = rainix.devShells.${system}.default.nativeBuildInputs;
        };
      }
    );

}

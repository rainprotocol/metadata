{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    foundry.url = "github:shazow/foundry.nix/main";
  };

  outputs = { self, flake-utils, nixpkgs, rust-overlay, foundry }:

  flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [ (import rust-overlay) foundry.overlay ];

      pkgs = (import nixpkgs) {
        inherit system overlays;
      };

      rust-version = "1.75.0";

      rust-toolchain = pkgs.rust-bin.stable.${rust-version}.default;

    in rec {
      # For `nix build` & `nix run`:
      defaultPackage = (pkgs.makeRustPlatform{
        rustc = rust-toolchain;
        cargo = rust-toolchain;
      }).buildRustPackage {
        src = ./.;
        doCheck = false;
        name = "rain-meta";
        cargoLock.lockFile = ./Cargo.lock;
        # allows for git deps to be resolved without the need to specify their outputHash
        cargoLock.allowBuiltinFetchGit = true;
        buildPhase = ''
          cargo build --release --bin rain-meta --all-features
        '';
        installPhase = ''
          mkdir -p $out/bin
          cp target/release/rain-meta $out/bin/
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

      # For `nix develop`:
      devShell = pkgs.mkShell {
        nativeBuildInputs = (with pkgs; [ 
          openssl 
          pkg-config
          foundry-bin
          rust-toolchain
          slither-analyzer 
        ] ++ lib.optionals stdenv.isDarwin [
          darwin.apple_sdk.frameworks.SystemConfiguration
        ]);
      };
    }
  );
}

{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    foundry.url = "github:shazow/foundry.nix/monthly";
  };

  outputs = { self, flake-utils, naersk, nixpkgs, rust-overlay, foundry }:

  flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [ (import rust-overlay) foundry.overlay ];

      pkgs = (import nixpkgs) {
        inherit system overlays;
      };

      rust-version = "1.75.0";

      rust-toolchain = pkgs.rust-bin.stable.${rust-version}.default;

      forge-bin = "${foundry.defaultPackage.${system}}/bin/forge";

      naersk' = pkgs.callPackage naersk {
        rustc = rust-toolchain;
        cargo = rust-toolchain;
      };

    in rec {
      # For `nix build` & `nix run`:
      defaultPackage = naersk'.buildPackage {
        src = ./.;
        nativeBuildInputs = (with pkgs; [ 
          gmp
          openssl 
          pkg-config
        ] ++ lib.optionals stdenv.isDarwin [
          darwin.apple_sdk.frameworks.SystemConfiguration
        ]);
        cargoBuildOptions = (prev: prev ++ [ "--all-features" ]);
      };

      # For `nix develop`:
      devShell = pkgs.mkShell {
        nativeBuildInputs = (with pkgs; [ 
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

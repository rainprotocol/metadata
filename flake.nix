{
  description = "Flake for development workflows.";

  inputs = {
    rainix.url = "github:rainprotocol/rainix/3bbc03daed3fca4e6fa837f4f0c948441d1aeff5";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {self, flake-utils, rainix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = rainix.pkgs.${system};
      in {
        packages = rainix.packages.${system};
        devShells = rainix.devShells.${system};
      }
    );

}

let
  pkgs = import
    (builtins.fetchTarball {
      name = "nixos-unstable-2021-10-01";
      url = "https://github.com/nixos/nixpkgs/archive/d3d2c44a26b693293e8c79da0c3e3227fc212882.tar.gz";
      sha256 = "0vi4r7sxzfdaxzlhpmdkvkn3fjg533fcwsy3yrcj5fiyqip2p3kl";
    })
    { };

  prettier-check = pkgs.writeShellScriptBin "prettier-check" ''
    prettier --check .
  '';

  prettier-write = pkgs.writeShellScriptBin "prettier-write" ''
    prettier --write .
  '';

  flush-all = pkgs.writeShellScriptBin "flush-all" ''
    rm -rf cache
    rm -rf node_modules
    rm -rf abis
    rm -rf artifacts
    rm -rf build
    rm -rf contracts
    rm -rf generated
    rm -rf typechain
  '';

  init = pkgs.writeShellScriptBin "init" ''
    npm install
  '';

  codegen = pkgs.writeShellScriptBin "codegen" ''
    graph codegen
  '';

  build = pkgs.writeShellScriptBin "build" ''
    graph build
  '';

  docker-up = pkgs.writeShellScriptBin "docker-up" ''
    docker-compose -f docker/docker-compose.yml up --build -d
  '';

  docker-down = pkgs.writeShellScriptBin "docker-down" ''
    docker-compose -f docker/docker-compose.yml down
  '';

  ci-test = pkgs.writeShellScriptBin "ci-test" ''
    npx mustache config/localhost.json subgraph.template.yaml subgraph.yaml
    codegen
    npm run test
  '';

  compile = pkgs.writeShellScriptBin "compile" ''
    hardhat compile --force
  '';

  ci-prepare-subgraph = pkgs.writeShellScriptBin "ci-prepair-subgraph" ''
    npx mustache config/localhost.json subgraph.template.yaml subgraph.yaml
  '';

in
pkgs.stdenv.mkDerivation {
  name = "shell";
  buildInputs = [
    pkgs.nixpkgs-fmt
    pkgs.yarn
    pkgs.nodejs-16_x
    prettier-check
    prettier-write
    flush-all
    init
    codegen
    compile
    build
    ci-test
    docker-up
    docker-down
    ci-prepare-subgraph
  ];

  shellHook = ''
    export PATH=$( npm bin ):$PATH
  '';
}
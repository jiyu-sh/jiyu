{
  description = "Freedom of managing proxies.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    flake-parts.url = "github:hercules-ci/flake-parts";

    systems.url = "github:nix-systems/default";

    crane.url = "github:ipetkov/crane";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-parts,
      systems,
      crane,
      rust-overlay,
      ...
    }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import systems;

      perSystem =
        {
          system,
          ...
        }:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              rust-overlay.overlays.default
            ];
          };

          lib = pkgs.lib;

          rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

          craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

          cargoToml = fromTOML (builtins.readFile ./Cargo.toml);

          version = cargoToml.workspace.package.version;

          src = lib.cleanSource ./.;

          cargoSrc = craneLib.cleanCargoSource src;

          rustBuildInputs = [
            pkgs.openssl
            pkgs.pkg-config
          ];

          nativeBuildInputs = [ pkgs.pkg-config ];

          cargoArtifacts = craneLib.buildDepsOnly {
            pname = "jiyu";

            inherit version;

            src = cargoSrc;

            strictDeps = true;

            buildInputs = rustBuildInputs;

            inherit nativeBuildInputs;
          };

          jiyu-cli = pkgs.callPackage ./default.nix {
            inherit craneLib; # for `buildPackage`

            name = "jiyu-cli";

            inherit version;

            binary = "jiyu";

            inherit cargoArtifacts; # for caching
          };
        in
        {
          packages = {
            inherit jiyu-cli;
            default = jiyu-cli;
          };

          checks = {
            inherit jiyu-cli;
          };

          devShells.default = pkgs.mkShell {
            name = "jiyu-dev";
            buildInputs = rustBuildInputs;
            nativeBuildInputs = [ rustToolchain ];
          };
        };
    };
}

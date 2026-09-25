{
  description = "Freedom of ...";

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

          rustBuildInputs = [
            pkgs.openssl
            pkgs.pkg-config
          ];

          rustNativeBuildInputs = [
            pkgs.pkg-config
          ];

          cargoToml = fromTOML (builtins.readFile ./Cargo.toml);

          fullSource = lib.cleanSource ./.;

          cargoSource = craneLib.cleanCargoSource fullSource;

          commonArgs = {
            src = fullSource;
            strictDeps = true;
            buildInputs = rustBuildInputs;
            nativeBuildInputs = rustNativeBuildInputs;
          };

          depsArgs = {
            pname = "jiyu-deps";
            src = cargoSource;
            version = cargoToml.workspace.package.version;
          };

          cargoArtifacts = craneLib.buildDepsOnly (commonArgs // depsArgs);

          rustPackage =
            package:
            {
              binary ? package,
              features ? [ ],
            }:
            let
              version = cargoToml.workspace.package.version;
              featureFlags = lib.concatStringsSep " " (map (feature: "--features ${feature}") features);
              extraFlags = "--locked --package ${package} ${featureFlags}";
              installPhase = ''
                mkdir -p $out/bin
                mv target/release/${binary} $out/bin/
              '';

              args = {
                inherit version;
                pname = package;

                inherit cargoArtifacts;
                cargoExtraArgs = extraFlags;

                installPhaseCommand = installPhase;
              };
            in
            craneLib.buildPackage (commonArgs // args);

          jiyu-cli = rustPackage "jiyu-cli" {
            binary = "jiyu";
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

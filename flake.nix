{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    crane.url = "github:ipetkov/crane";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;

        overlays = [
          (import rust-overlay)

          (final: prev: {
            rustToolchain = prev.rust-bin.stable.latest.default;
            craneLib = (crane.mkLib prev).overrideToolchain final.rustToolchain;
          })
        ];
      };

      checks = pkgs.callPackage ./nix/checks.nix {};
    in {
      checks = {
        inherit (checks) clippy fmt test;
      };

      devShells.default = pkgs.callPackage ./nix/dev.nix {};

      formatter = pkgs.alejandra;
    });
}

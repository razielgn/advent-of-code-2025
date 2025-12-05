{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;

        overlays = [(import rust-overlay)];
      };

      rustToolchain = pkgs.rust-bin.stable.latest.default;
    in {
      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          rustToolchain
          cargo-aoc
          cargo-outdated
        ];
      };

      formatter = pkgs.alejandra;
    });
}

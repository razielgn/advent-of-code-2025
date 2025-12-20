{
  clang,
  craneLib,
  lib,
  pkg-config,
  z3,
}: let
  src = lib.cleanSourceWith {
    src = craneLib.path ../.;

    filter = path: type:
      (lib.hasSuffix "Cargo.toml" path)
      || (lib.hasSuffix "Cargo.lock" path)
      || (lib.hasSuffix "rustfmt.toml" path)
      || (lib.hasSuffix ".txt" path)
      || (craneLib.filterCargoSources path type);
  };

  commonArgs = {
    inherit src;

    nativeBuildInputs = [
      pkg-config
    ];

    buildInputs = [
      z3
    ];

    env = {
      LIBCLANG_PATH = "${lib.getLib clang.cc}/lib";
    };
  };

  cargoArtifacts = craneLib.buildDepsOnly (commonArgs
    // {
      doCheck = false;
    });
in {
  clippy = craneLib.cargoClippy (commonArgs
    // {
      inherit cargoArtifacts;

      pname = "aoc-2025-clippy";
      cargoClippyExtraArgs = "--all-targets -- --deny warnings -W clippy::all -W clippy::pedantic -W clippy::nursery";
    });

  fmt = craneLib.cargoFmt (commonArgs
    // {
      pname = "aoc-2025-fmt";
    });

  test = craneLib.cargoNextest (commonArgs
    // {
      inherit cargoArtifacts;

      pname = "aoc-2025-test";

      partitions = 1;
      partitionType = "count";
    });
}

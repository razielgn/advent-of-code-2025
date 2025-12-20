{
  actionlint,
  cargo-aoc,
  cargo-flamegraph,
  cargo-outdated,
  clang,
  lib,
  mkShell,
  pkg-config,
  python3,
  rustToolchain,
  z3,
}:
mkShell {
  LIBCLANG_PATH = "${lib.getLib clang.cc}/lib";

  packages = [
    actionlint
    cargo-aoc
    cargo-flamegraph
    cargo-outdated
    pkg-config
    rustToolchain
    z3
    (python3.withPackages (ps:
      with ps; [
        matplotlib
        numpy
        ruff
      ]))
  ];
}

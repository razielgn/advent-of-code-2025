{
  actionlint,
  cargo-aoc,
  cargo-outdated,
  cargo-flamegraph,
  mkShell,
  python3,
  rustToolchain,
  z3,
}:
mkShell {
  packages = [
    actionlint
    cargo-aoc
    cargo-outdated
    cargo-flamegraph
    rustToolchain
    z3.dev
    (python3.withPackages (ps:
      with ps; [
        matplotlib
        numpy
        ruff
      ]))
  ];
}

{
  actionlint,
  cargo-aoc,
  cargo-outdated,
  cargo-flamegraph,
  mkShell,
  python3,
  rustToolchain,
}:
mkShell {
  packages = [
    actionlint
    cargo-aoc
    cargo-outdated
    cargo-flamegraph
    rustToolchain
    (python3.withPackages (ps:
      with ps; [
        matplotlib
        numpy
        ruff
      ]))
  ];
}

{
  actionlint,
  cargo-aoc,
  cargo-outdated,
  mkShell,
  rustToolchain,
}:
mkShell {
  packages = [
    actionlint
    cargo-aoc
    cargo-outdated
    rustToolchain
  ];
}

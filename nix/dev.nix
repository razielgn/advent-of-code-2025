{
  actionlint,
  cargo-aoc,
  cargo-outdated,
  mkShell,
  python3,
  rustToolchain,
}:
mkShell {
  packages = [
    actionlint
    cargo-aoc
    cargo-outdated
    rustToolchain
    (python3.withPackages (ps:
      with ps; [
        matplotlib
        numpy
        ruff
      ]))
  ];
}

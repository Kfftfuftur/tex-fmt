{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  inputsFrom = [(pkgs.callPackage ./default.nix {})];
  buildInputs = let
    python = pkgs.python3.withPackages (ps:
      with ps; [
        grip
        matplotlib
        pillow
      ]);
  in [
    pkgs.bacon
    pkgs.cacert
    pkgs.cargo-flamegraph
    pkgs.cargo-edit
    pkgs.clippy
    pkgs.diff-so-fancy
    pkgs.hyperfine
    pkgs.poppler_utils
    python
    pkgs.rustfmt
    pkgs.shellcheck
    pkgs.texlive.combined.scheme-full
  ];
}

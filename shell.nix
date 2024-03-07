with import <nixpkgs> {};
mkShell {
  packages = [
    cargo
    rustc
    rust-analyzer
    pkg-config
    guile
    rustPlatform.bindgenHook
    rustfmt
  ];
  env.RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
}


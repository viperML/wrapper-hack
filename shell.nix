let
  rust-overlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
  pkgs = import <nixpkgs> { overlays = [ (import rust-overlay) ]; };
  toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
in
with pkgs;
mkShell {
  packages = [
    toolchain
    clang-tools
    bear
  ];
  env.RUST_SRC_PATH =  "${toolchain}/lib/rustlib/src/rust/library";
}

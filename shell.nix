{ pkgs ? import <nixpkgs> {}, }:

with pkgs;
let 
  toolchain = with pkgs.fenix; combine [
    complete.toolchain
    targets.x86_64-unknown-linux-musl.latest.rust-std
  ]; 
in mkShell {
  buildInputs = with pkgs; [
    toolchain
    mustache-go
  ];

  shellHook = ''
    export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
  '';
}

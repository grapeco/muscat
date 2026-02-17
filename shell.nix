{ pkgs ? import <nixpkgs> {}, }:

with pkgs;
let 
  depend = with pkgs; [
    (fenix.combine [
      fenix.complete.toolchain
      fenix.targets.x86_64-unknown-linux-musl.latest.rust-std
    ])
    
    wayland
    libxkbcommon
  ]; 
in mkShell {
  buildInputs = depend;

  shellHook = ''
    export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
  '';
  
  LD_LIBRARY_PATH = lib.makeLibraryPath depend;
  WINIT_UNIX_BACKEND = "wayland";
}

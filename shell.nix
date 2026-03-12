{ pkgs ? import <nixpkgs> {}, }:

with pkgs;
let 
  depend = with pkgs; [
    (fenix.combine [
      fenix.complete.toolchain
    ])
    
    wayland
    libxkbcommon
  ]; 
in mkShell {
  buildInputs = depend;

  shellHook = ''
    export PATH=$PATH:''${CARGO_HOME:~/.cargo}/bin
  '';
  
  LD_LIBRARY_PATH = lib.makeLibraryPath depend;
}

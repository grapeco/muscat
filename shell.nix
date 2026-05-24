{ pkgs ? import <nixpkgs> {} }:

with pkgs;
mkShell rec {
  nativeBuildInputs = [ 
    pkg-config 
    wrapGAppsHook3
  ];
  
  buildInputs = with pkgs; [
    (fenix.combine [
      fenix.complete.toolchain
    ])
    
    gtk3
    glib
  ]; 

  shellHook = ''
    export PATH=$PATH:''${CARGO_HOME:~/.cargo}/bin
  '';
  
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}

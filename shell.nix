{ pkgs ? import <nixpkgs> {} }:

with pkgs;
mkShell rec {
  nativeBuildInputs = [ 
    pkg-config 
    wrapGAppsHook3
  ];
  
  buildInputs = with pkgs; [
    (fenix.combine [
      fenix.minimal.toolchain
    ])
    
    gtk3
    glib
  ]; 

  shellHook = ''
    export PATH=$PATH:''${CARGO_HOME:~/.cargo}/bin
    export XDG_DATA_DIRS="${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}"
  '';
  
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}

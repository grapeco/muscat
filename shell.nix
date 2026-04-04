{ pkgs ? import <nixpkgs> {} }:

with pkgs;
let 
  depend = with pkgs; [
    (fenix.combine [
      fenix.complete.toolchain
    ])
    
    gtk3
    papirus-icon-theme
    glib
  ]; 
in mkShell {
  nativeBuildInputs = [ 
    pkg-config 
    wrapGAppsHook3
  ];
  buildInputs = depend;

  shellHook = ''
    export PATH=$PATH:''${CARGO_HOME:~/.cargo}/bin
    export XDG_DATA_DIRS=$GSETTINGS_SCHEMAS_PATH
  '';
  
  LD_LIBRARY_PATH = lib.makeLibraryPath depend;
}

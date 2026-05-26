{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { nixpkgs, fenix, flake-utils, ... }:
  flake-utils.lib.eachDefaultSystem (system:
  let
    overlays = [ fenix.overlays.default ];
    pkgs = import nixpkgs { inherit system overlays; };
    shell = import ./shell.nix { inherit pkgs; };
  in {
    devShells.default = shell;
    packages.default = pkgs.rustPlatform.buildRustPackage {
      name = "muscat";
      src = ./.;
      cargoLock.lockFile = ./Cargo.lock;
      nativeBuildInputs = shell.nativeBuildInputs;
      buildInputs = shell.buildInputs;
    };
  });
}

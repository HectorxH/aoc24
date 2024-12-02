{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    nixpkgs-unstable.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      nixpkgs-unstable,
      utils,
      ...
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        pkgs-unstable = import nixpkgs-unstable { inherit system; };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs-unstable; [
            cargo
            rustc
            rustfmt
            rustPackages.clippy
            rust-analyzer
          ];
          shellHook = ''
            exec zsh
          '';
        };
      }
    );
}

{
  description = "LESS development shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in {
        devShells.default = pkgs.mkShell {
          name = "devshell";

          buildInputs = with pkgs; [
            gcc
            pkg-config
            git
            rustc
            rustfmt
            python3
            cargo
            clippy
          ];

          # shellHook = ''
          # '';
        };
      }
    );
}
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
            xorg.libX11
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXinerama
            xorg.libXi 
            libxkbcommon
            libGL
            pkg-config
            git
            rustc
            rustfmt
            python3
            cargo
            clippy
          ];
          shellHook = ''
            export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [
              pkgs.xorg.libX11
              pkgs.xorg.libXcursor
              pkgs.xorg.libXrandr
              pkgs.xorg.libXinerama
              pkgs.xorg.libXi
              pkgs.libxkbcommon
              pkgs.libGL
            ]}:$LD_LIBRARY_PATH
          '';
        };
      }
    );
}
{
  description = "Arkyo Http Server";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = inputs@ { self, nixpkgs, flake-parts, ... } :
  flake-parts.lib.mkFlake { inherit inputs; } {
    systems = [ "x86_64-linux" "aarch64-linux" ];

    perSystem = { pkgs, system, lib, ... } : {

      devShells.default = pkgs.mkShell rec {
        buildInputs = with pkgs; [
          xorg.libXrender xorg.libXrandr
          xorg.libXcursor xorg.libXi
          xorg.libXinerama libGL
        ];

        LD_LIBRARY_PATH =
          lib.makeLibraryPath buildInputs;

        packages = with pkgs; [
          clippy bacon rust-analyzer
          cargo rustc rustfmt cmake
        ];
      };

    };
  };
}


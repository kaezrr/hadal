{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.zst";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { nixpkgs, fenix, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      fenixLib = fenix.packages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = [
          fenixLib.complete.toolchain
        ];

        env.RUSTFLAGS = "-C link-args=-Wl,-rpath,${
          pkgs.lib.makeLibraryPath [
            pkgs.wayland
            pkgs.libxkbcommon
            pkgs.vulkan-loader
          ]
        }";
      };
    };
}

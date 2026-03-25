{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    common.url = "github:liar2357/nix-dev-common"; # または Git URL
  };

  outputs =
    {
      self,
      nixpkgs,
      common,
    }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      shells = common.devShells.${system};
    in
    {
      devShells.${system} = {
        default = shells.rust_gtk_depnds;
      };
    };
}

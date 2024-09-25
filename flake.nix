{
  description = "dusk";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
  }:
    utils.lib.eachDefaultSystem
    (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      packages = {
        default = pkgs.rustPlatform.buildRustPackage {
          name = "dusk";

          src = ./.;

          cargoHash = "sha256-pGwDmd6E2GX726NXgq36laIRKjHZZXTQccZBWOp/HAM=";
        };
      };

      apps.default = utils.lib.mkApp {drv = self.packages.${system}.default;};
    });
}

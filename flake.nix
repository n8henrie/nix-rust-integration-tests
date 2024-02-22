{
  description = "Rust integration tests requiring localhost aren't working on MacOS";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/release-23.11";
  outputs = {
    self,
    nixpkgs,
  }: let
    systems = ["aarch64-darwin" "x86_64-linux" "aarch64-linux"];
    forAllSystems = func: nixpkgs.lib.genAttrs systems (system: func (import nixpkgs {inherit system;}));
  in {
    packages = forAllSystems (pkgs: {
      default = pkgs.rustPlatform.buildRustPackage {
        name = "foo";
        version = "0.0.1";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
      };
    });
  };
}

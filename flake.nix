{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      fenix,
      naersk,
    }:
    flake-utils.lib.eachDefaultSystem (system: {
      defaultPackage =
        let
          target = "armv7-unknown-linux-gnueabihf";
          pkgs = nixpkgs.legacyPackages.${system};
          toolchain =
            with fenix.packages.${system};
            combine [
              minimal.cargo
              minimal.rustc
              targets.${target}.latest.rust-std
            ];
        in
        (naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        }).buildPackage
          {
            src = ./.;
            CARGO_BUILD_TARGET = target;
            CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER =
              let
                inherit (pkgs.pkgsCross.armv7l-hf-multiplatform.stdenv) cc;
              in
              "${cc}/bin/${cc.targetPrefix}cc";
          };
    });
}

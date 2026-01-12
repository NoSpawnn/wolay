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
          target = "armv7-unknown-linux-musleabihf";
          pkgs = nixpkgs.legacyPackages.${system};
          cross = pkgs.pkgsCross.muslpi;
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
            CC_armv7_unknown_linux_musleabihf = "${cross.stdenv.cc}/bin/${cross.stdenv.cc.targetPrefix}cc";
            AR_armv7_unknown_linux_musleabihf = "${cross.stdenv.cc.bintools}/bin/${cross.stdenv.cc.targetPrefix}ar";
            CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER = "${cross.stdenv.cc}/bin/${cross.stdenv.cc.targetPrefix}cc";
          };
    });
}

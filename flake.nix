{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
    flake-parts.url = "github:hercules-ci/flake-parts";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      flake-parts,
      naersk,
      fenix,
      ...
    }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        # "x86_64-darwin"
      ];

      perSystem =
        { pkgs, system, ... }:
        let
          naersk-lib = pkgs.callPackage naersk { };

          armv7 = {
            target = "armv7-unknown-linux-musleabihf";
            crossPkgs = pkgs.pkgsCross.muslpi;
            toolchain =
              with fenix.packages.${system};
              combine [
                minimal.cargo
                minimal.rustc
                targets.${armv7.target}.latest.rust-std
              ];
          };
        in
        {
          devShells.default = pkgs.mkShell {
            buildInputs = with pkgs; [
              cargo
              rustc
              rustfmt
              rustPackages.clippy
            ];
            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
          };

          packages.default = naersk-lib.buildPackage ./.;

          packages.${armv7.target} =
            (naersk.lib.${system}.override {
              cargo = armv7.toolchain;
              rustc = armv7.toolchain;
            }).buildPackage
              {
                src = ./.;
                CARGO_BUILD_TARGET = armv7.target;
                CC_armv7_unknown_linux_musleabihf = "${armv7.crossPkgs.stdenv.cc}/bin/${armv7.crossPkgs.stdenv.cc.targetPrefix}cc";
                AR_armv7_unknown_linux_musleabihf = "${armv7.crossPkgs.stdenv.cc.bintools}/bin/${armv7.crossPkgs.stdenv.cc.targetPrefix}ar";
                CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER = "${armv7.crossPkgs.stdenv.cc}/bin/${armv7.crossPkgs.stdenv.cc.targetPrefix}cc";
                RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
              };
        };
    };
}

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
      nixpkgs,
      flake-parts,
      naersk,
      fenix,
      ...
    }@inputs:
    let
      pkgs = import nixpkgs;
    in
    {
      devShell =
        with pkgs;
        mkShell {
          buildInputs = [
            cargo
            rustc
            rustfmt
            rustPackages.clippy
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
    }
    // (flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
        "armv7-unknown-linux-musleabihf"
      ];

      perSystem =
        { pkgs, system, ... }:
        let
          naersk-lib = pkgs.callPackage naersk { };

          armv7Target = "armv7-unknown-linux-musleabihf";
          armv7CrossPkgs = pkgs.pkgsCross.muslpi;
          armv7Toolchain =
            with fenix.packages.${system};
            combine [
              minimal.cargo
              minimal.rustc
              targets.${armv7Target}.latest.rust-std
            ];
        in
        {
          packages.default = naersk-lib.buildPackage ./.;
          packages.armv7 =
            (naersk.lib.${system}.override {
              cargo = armv7Toolchain;
              rustc = armv7Toolchain;
            }).buildPackage
              {
                src = ./.;
                CARGO_BUILD_TARGET = armv7Target;
                CC_armv7_unknown_linux_musleabihf = "${armv7CrossPkgs.stdenv.cc}/bin/${armv7CrossPkgs.stdenv.cc.targetPrefix}cc";
                AR_armv7_unknown_linux_musleabihf = "${armv7CrossPkgs.stdenv.cc.bintools}/bin/${armv7CrossPkgs.stdenv.cc.targetPrefix}ar";
                CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER = "${armv7CrossPkgs.stdenv.cc}/bin/${armv7CrossPkgs.stdenv.cc.targetPrefix}cc";
                RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
              };
        };
    });
}

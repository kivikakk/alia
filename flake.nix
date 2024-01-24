{
  description = "alia";

  outputs = inputs @ {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
      inherit (pkgs) lib;
      cargoToml = lib.importTOML ./Cargo.toml;
    in rec {
      formatter = pkgs.alejandra;

      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = cargoToml.package.name;
        version = cargoToml.package.version;
        src = ./.;

        cargoLock.lockFile = ./Cargo.lock;

        nativeBuildInputs = [
          pkgs.pkg-config
        ];

        buildInputs =
          [
            pkgs.openssl
          ]
          ++ lib.optionals (pkgs.stdenv.isDarwin) [
            pkgs.darwin.apple_sdk.frameworks.Security
          ];
      };

      devShells.default = packages.default.overrideAttrs (finalAttrs: prevAttrs: {
        nativeBuildInputs =
          prevAttrs.nativeBuildInputs
          ++ [
            pkgs.rust-analyzer
            pkgs.rustfmt
            pkgs.lldb_16
            pkgs.re2c
            pkgs.clippy
          ];
      });

      nixosModules.default = {...}: {
        home-manager.sharedModules = [(import ./home.nix {inherit self system;})];
      };

      darwinModules.default = {...}: {
        home-manager.sharedModules = [(import ./home.nix {inherit self system;})];
      };
    });
}

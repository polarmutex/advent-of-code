{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = {
    self,
    nixpkgs,
    advisory-db,
    crane,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachSystem [
      "x86_64-linux"
    ]
    (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };

      inherit (pkgs) lib;

      rustTarget = pkgs.rust-bin.nightly."2023-10-01".default.override {
        extensions = ["rust-src" "rust-analyzer" "clippy"];
      };

      craneLib = crane.lib.${system};
      src = craneLib.cleanCargoSource ./.;

      buildInputs = with pkgs;
        [
          # Add additional build inputs here
          pkg-config
          openssl.dev
        ]
        ++ lib.optionals pkgs.stdenv.isDarwin [
          # Additional darwin specific inputs can be set here
          pkgs.libiconv
        ];

      # Build *just* the cargo dependencies, so we can reuse
      # all of that work (e.g. via cachix) when running in CI
      cargoArtifacts = craneLib.buildDepsOnly {
        inherit src buildInputs;
      };

      # Build the actual crate itself, reusing the dependency
      # artifacts from above.
      my-crate = craneLib.buildPackage {
        inherit cargoArtifacts src buildInputs;
      };
    in {
      checks =
        {
          # Build the crate as part of `nix flake check` for convenience
          inherit my-crate;

          # Run clippy (and deny all warnings) on the crate source,
          # again, resuing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          my-crate-clippy = craneLib.cargoClippy {
            inherit cargoArtifacts src buildInputs;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          };

          my-crate-doc = craneLib.cargoDoc {
            inherit cargoArtifacts src;
          };

          # Check formatting
          my-crate-fmt = craneLib.cargoFmt {
            inherit src;
          };

          # Audit dependencies
          my-crate-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };

          # Run tests with cargo-nextest
          # Consider setting `doCheck = false` on `my-crate` if you do not want
          # the tests to run twice
          my-crate-nextest = craneLib.cargoNextest {
            inherit cargoArtifacts src buildInputs;
            partitions = 1;
            partitionType = "count";
          };
        }
        // lib.optionalAttrs (system == "x86_64-linux") {
          # NB: cargo-tarpaulin only supports x86_64 systems
          # Check code coverage (note: this will not upload coverage anywhere)
          my-crate-coverage = craneLib.cargoTarpaulin {
            inherit cargoArtifacts src;
          };
        };

      packages.default = my-crate;

      apps.default = flake-utils.lib.mkApp {
        drv = my-crate;
      };

      devShells.default = pkgs.mkShell {
        inputsFrom = builtins.attrValues self.checks;

        LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
        Z3_SYS_Z3_HEADER = "${pkgs.z3.dev}/include/z3.h";
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
          "${pkgs.z3.lib}"
          # pkgs.stdenv.cc.cc
          # Add any missing library needed
          # You can use the nix-index package to locate them, e.g. nix-locate -w --top-level --at-root /lib/libudev.so.1
        ];

        # Extra inputs can be added here
        nativeBuildInputs = with pkgs; [
          aoc-cli
          cargo
          cargo-watch
          #rustc
          rustTarget
          clippy
          just
          rustfmt
          pkg-config
          openssl.dev
          nodePackages.prettier
          clang
          z3.lib
        ];
      };
    });
}

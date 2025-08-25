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
    };

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
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

      rustTarget = pkgs.rust-bin.nightly."2024-11-01".default.override {
        extensions = ["rust-src" "rust-analyzer" "clippy"];
      };

      craneLib = crane.mkLib pkgs;
      # Only keeps markdown files
      textFilter = path: _type: builtins.match ".*txt$" path != null;
      textOrCargo = path: type:
        (textFilter path type) || (craneLib.filterCargoSources path type);
      src = lib.cleanSourceWith {
        src = ./.;
        filter = textOrCargo;
        name = "source";
      };
      # src = craneLib.cleanCargoSource ./.;

      # Common arguments can be set here to avoid repeating them later
      commonArgs = {
        inherit src;
        strictDeps = true;
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
      };

      craneLibLLvmTools =
        craneLib.overrideToolchain
        (pkgs.rust-bin.nightly."2024-11-01".default.override {
          extensions = ["rust-src" "rust-analyzer" "clippy"];
        });

      # Build *just* the cargo dependencies (of the entire workspace),
      # so we can reuse all of that work (e.g. via cachix) when running in CI
      # It is *highly* recommended to use something like cargo-hakari to avoid
      # cache misses when building individual top-level-crates
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      individualCrateArgs =
        commonArgs
        // {
          inherit cargoArtifacts;
          inherit (craneLib.crateNameFromCargoToml {inherit src;}) version;
          # NB: we disable tests since we'll run them all via cargo-nextest
          doCheck = false;
        };

      fileSetForCrate = crate:
        lib.fileset.toSource {
          root = ./.;
          fileset = lib.fileset.unions [
            ./Cargo.toml
            ./Cargo.lock
            # (craneLib.fileset.commonCargoSources ./aoc_2024)
            # (craneLib.fileset.commonCargoSources ./framework)
            # (craneLib.fileset.commonCargoSources ./common)
            # (craneLib.fileset.commonCargoSources ./aoc_lib)
            (craneLib.fileset.commonCargoSources crate)
            # ./framework/template.txt
          ];
        };
      # Build the top-level crates of the workspace as individual derivations.
      # This allows consumers to only depend on (and build) only what they need.
      # Though it is possible to build the entire workspace as a single derivation,
      # so this is left up to you on how to organize things
      #
      # Note that the cargo workspace must define `workspace.members` using wildcards,
      # otherwise, omitting a crate (like we do below) will result in errors since
      # cargo won't be able to find the sources for all members.
      # aoc_framework = craneLib.buildPackage (individualCrateArgs
      #   // {
      #     pname = "aoc-framework";
      #     cargoExtraArgs = "-p framework";
      #     src = fileSetForCrate ./framework;
      #   });
    in {
      checks =
        {
          # Build the crate as part of `nix flake check` for convenience
          # inherit aoc_framework;

          # Run clippy (and deny all warnings) on the workspace source,
          # again, reusing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          my-workspace-clippy = craneLib.cargoClippy (commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            });

          my-workspace-doc = craneLib.cargoDoc (commonArgs
            // {
              inherit cargoArtifacts;
            });

          # Check formatting
          my-workspace-fmt = craneLib.cargoFmt {
            inherit src;
          };

          my-workspace-toml-fmt = craneLib.taploFmt {
            src = pkgs.lib.sources.sourceFilesBySuffices src [".toml"];
            # taplo arguments can be further customized below as needed
            # taploExtraArgs = "--config ./taplo.toml";
          };

          # Audit dependencies
          my-workspace-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };

          # Audit licenses
          # my-workspace-deny = craneLib.cargoDeny {
          #   inherit src;
          # };

          # Run tests with cargo-nextest
          # Consider setting `doCheck = false` on other crate derivations
          # if you do not want the tests to run twice
          my-workspace-nextest = craneLib.cargoNextest (commonArgs
            // {
              inherit cargoArtifacts;
              partitions = 1;
              partitionType = "count";
            });
        }
        // lib.optionalAttrs (system == "x86_64-linux") {
          # NB: cargo-tarpaulin only supports x86_64 systems
          # Check code coverage (note: this will not upload coverage anywhere)
          my-crate-coverage = craneLib.cargoTarpaulin {
            inherit cargoArtifacts src;
          };
        };

      packages =
        {
          # default = aoc_framework;
        }
        // lib.optionalAttrs (!pkgs.stdenv.isDarwin) {
          my-workspace-llvm-coverage = craneLibLLvmTools.cargoLlvmCov (commonArgs
            // {
              inherit cargoArtifacts;
            });
          aoc-2024-test = craneLib.cargoTest (commonArgs
            // {
              inherit cargoArtifacts;
              cargoExtraArgs = "-p aoc_2024";
              # partitions = 1;
              # partitionType = "count";
            });
        };

      apps = {
        default = flake-utils.lib.mkApp {
          # drv = aoc_framework;
        };
      };

      devShells.default = craneLib.devShell {
        # Inherit inputs from checks.
        checks = self.checks.${system};

        # Extra inputs can be added here; cargo and rustc are provided by default.
        packages = [
        ];
      };
      #   devShells.default = pkgs.mkShell {
      #     inputsFrom = builtins.attrValues self.checks;
      #
      #     LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
      #     Z3_SYS_Z3_HEADER = "${pkgs.z3.dev}/include/z3.h";
      #     LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
      #       "${pkgs.z3.lib}"
      #       # pkgs.stdenv.cc.cc
      #       # Add any missing library needed
      #       # You can use the nix-index package to locate them, e.g. nix-locate -w --top-level --at-root /lib/libudev.so.1
      #     ];
      #
      #     # Extra inputs can be added here
      #     nativeBuildInputs = with pkgs; [
      #       aoc-cli
      #       cargo
      #       cargo-watch
      #       #rustc
      #       rustTarget
      #       clippy
      #       just
      #       rustfmt
      #       pkg-config
      #       openssl.dev
      #       nodePackages.prettier
      #       clang
      #       z3.lib
      #     ];
      #   };
    });
}

{
  description = "modeling-api development environment";

  # Flake inputs
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay"; # A helper for Rust + Nix
  };

  # Flake outputs
  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }: let
    # Overlays enable you to customize the Nixpkgs attribute set
    overlays = [
      # Makes a `rust-bin` attribute available in Nixpkgs
      (import rust-overlay)
      # Provides a `rustToolchain` attribute for Nixpkgs that we can use to
      # create a Rust environment
      (self: super: {
        rustToolchain = super. rust-bin.stable.latest.default.override {
          extensions = ["rustfmt" "llvm-tools-preview"];
        };

        # stand-alone nightly formatter so we get the fancy unstable flags
        nightlyRustfmt = super.rust-bin.selectLatestNightlyWith (toolchain:
          toolchain.default.override {
            extensions = ["rustfmt"]; # just the formatter
          });
      })
    ];

    # Systems supported
    allSystems = [
      "x86_64-linux" # 64-bit Intel/AMD Linux
      "aarch64-linux" # 64-bit ARM Linux
      "x86_64-darwin" # 64-bit Intel macOS
      "aarch64-darwin" # 64-bit ARM macOS
    ];

    # Helper to provide system-specific attributes
    forAllSystems = f:
      nixpkgs.lib.genAttrs allSystems (system:
        f {
          pkgs = import nixpkgs {inherit overlays system;};
        });
  in {
    # Development environment output
    devShells = forAllSystems ({pkgs}: {
      default = pkgs.mkShell.override {stdenv = pkgs.clangStdenv;} {
        # The Nix packages provided in the environment
        packages = with pkgs; [
          # The package provided by our custom overlay. Includes cargo, Clippy, cargo-fmt,
          # rustdoc, rustfmt, and other tools.
          rustToolchain
          nightlyRustfmt

          # live reload
          bacon

          # cargo-llvm-cov
          cargo-nextest
          cargo-expand
          cargo-sort

          just

          # pyo3
          python3Full
        ];

        # needed for rustfmt-wrapper
        RUSTFMT = "${pkgs.nightlyRustfmt}/bin/rustfmt";
      };
    });
  };
}

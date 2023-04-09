{
  description = "hcc build flake";

  inputs = {

    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";     
    
  };


  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            pkg-config
            openssl
            openssl.bin
            pkg-config

            glibc
            nodejs
            wasm-pack
            binaryen

            rust-analyzer
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
              targets = [
                "x86_64-unknown-linux-gnu"  
                "wasm32-unknown-unknown"
              ];
            })

          ];

          shellHook = ''
                mkdir -p .secrets && touch .secrets/.env
                stat .secrets/jwtRS256.key > /dev/null || ssh-keygen -t rsa -b 4096 -m PEM -f ./.secrets/jwtRS256.key
                stat .secrets/jwtRS256.key.pub > /dev/null || openssl rsa -in jwtRS256.key -pubout -outform PEM -out ./.secrets/jwtRS256.key.pub

                stat hcc-db/.cargo-nix-local/bin/sea > /dev/null || cargo install sea-orm-cli --version '^0.8.1' --bin sea --root hcc-db/.cargo-nix-local/
        '';
        };
      }
    );

}
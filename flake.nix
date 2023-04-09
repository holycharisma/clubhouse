{
  description = "hcc build flake";

  inputs = {

    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    naersk.url = "github:nix-community/naersk";
    flake-utils.url  = "github:numtide/flake-utils";     
    
  };

  outputs = { self, nixpkgs, naersk, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        naersk' = pkgs.callPackage naersk {};
      in
      with pkgs;
      {
        devShell = mkShell {
          nativeBuildInputs = [
            rustc
            cargo
          ];
          buildInputs = [
            openssl
            openssl.bin
            pkg-config
          ];
        };

        defaultPackage = naersk'.buildPackage {
          src = ./.;
          buildInputs = [            
            openssl
            openssl.bin
            pkg-config
          ];
        };

      }
    );

}
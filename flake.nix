{
  inputs = {
    nix-eda.url = "github:fossi-foundation/nix-eda";
    flake-utils.url = "github:numtide/flake-utils";
  };
  
  outputs = { self, nix-eda, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nix-eda.inputs.nixpkgs {
          inherit system;
          overlays = [ nix-eda.overlays.default ];
        };
        pythonEnv = pkgs.python3.withPackages (ps: with ps; [
          pip setuptools virtualenv
        ]);
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pythonEnv
            pkgs.git
            pkgs.xschem  # Already configured with GUI support!
            pkgs.ngspice
            # All EDA tools from nix-eda work out-of-the-box
          ];
          
          shellHook = ''
            if [ ! -d .venv ]; then
              python -m venv .venv
            fi
            source .venv/bin/activate
            pip install -e .
          '';
        };
      }
    );
}
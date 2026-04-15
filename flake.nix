{
  description = "SSTADEx: Structured and Systematic Technology-Independent Analog Design Exploration";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        
        # Python con dependencias científicas desde Nix
        pythonEnv = pkgs.python3.withPackages (ps: with ps; [
          pip
          setuptools
          wheel
          virtualenv
          
          # Dependencias científicas (compiladas por Nix)
          numpy
          pandas
          sympy
          matplotlib
          scipy  # Por si lo necesitas
          
          # Jupyter/IPython
          ipykernel
          jupyter
          ipython
          
          # Herramientas de desarrollo
          pytest
          pytest-cov
          black
          flake8
          mypy
        ]);
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pythonEnv
            pkgs.git
            
            # Herramientas EDA para analog design
            pkgs.ngspice
            pkgs.xschem
            # pkgs.magic-vlsi  # Descomenta si lo necesitas
            # pkgs.gtkwave     # Descomenta si lo necesitas
            
            # Librerías del sistema necesarias
            pkgs.stdenv.cc.cc.lib  # Proporciona libstdc++
            pkgs.zlib
            
            # X11 para GUIs
            pkgs.xorg.libX11
            pkgs.xorg.libXext
            pkgs.xorg.libXrender
            pkgs.cairo
            pkgs.tcl
            pkgs.tk
          ];
          
          shellHook = ''
            # Configurar display para GUIs
            export DISPLAY=''${DISPLAY:-:0}
            
            # Configurar librerías del sistema
            export LD_LIBRARY_PATH=${pkgs.stdenv.cc.cc.lib}/lib:${pkgs.zlib}/lib:$LD_LIBRARY_PATH
            
            # Crear virtualenv con acceso a paquetes de Nix
            if [ ! -d .venv ]; then
              echo "📦 Creating virtual environment..."
              python -m venv .venv --system-site-packages
            fi
            
            # Activar virtualenv
            source .venv/bin/activate
            
            # Instalar dependencias que no están en Nix (SymMNA, mosplot)
            echo "🔧 Installing Git dependencies..."
            pip install --quiet \
              "SymMNA @ git+https://github.com/lild4d4/Symbolic-modified-nodal-analysis.git@cc6bd3f568ddcef69173fc2499dae33547f1620d" \
              "mosplot @ git+https://github.com/pmicgen/gmid.git@d5d3850a53841834949b866e934a15dbc570e72f"
            
            # Instalar tu paquete en modo editable (sin reinstalar deps de Nix)
            echo "📝 Installing sstadex in editable mode..."
            pip install -e . --no-deps
            
            # Registrar kernel de Jupyter
            python -m ipykernel install --user --name=sstadex --display-name="Python (SSTADEx)" 2>/dev/null || true
            
            # Información del ambiente
            echo ""
            echo "✨ SSTADEx Development Environment Ready!"
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            echo "🐍 Python: $(python --version)"
            echo "📊 NumPy: $(python -c 'import numpy; print(numpy.__version__)')"
            echo "📈 Pandas: $(python -c 'import pandas; print(pandas.__version__)')"
            echo "🔧 Virtual env: .venv"
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            echo ""
            echo "🚀 Ready to work! Your changes will be reflected immediately."
            echo "   Try: python test/pole/pole.py"
            echo ""
          '';
        };
        
        # Shell alternativo sin virtualenv (para CI/CD o testing)
        devShells.pure = pkgs.mkShell {
          buildInputs = [
            pythonEnv
            pkgs.git
            pkgs.ngspice
            pkgs.xschem
          ];
          
          shellHook = ''
            export LD_LIBRARY_PATH=${pkgs.stdenv.cc.cc.lib}/lib:$LD_LIBRARY_PATH
            echo "Pure Nix environment (no virtualenv)"
          '';
        };
      }
    );
}

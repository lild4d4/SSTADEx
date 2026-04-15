export HOME_DIR=$(pwd)

export PDK_ROOT=$HOME_DIR/IHP-Open-PDK
export PDK=ihp-sg13g2

# Python virtual environment
if [ ! -d "$HOME_DIR/.venv-sstadex" ]; then
    echo "Creando entorno virtual .venv-sstadex..."
    python3 -m venv "$HOME_DIR/.venv-sstadex"
fi

echo "Activando entorno virtual..."
source "$HOME_DIR/.venv-sstadex/bin/activate"

echo "Instalando dependencias desde pyproject.toml..."
pip install -e .

echo "Instalando submodulo gmid..."
pip install -e "$HOME_DIR/gmid"
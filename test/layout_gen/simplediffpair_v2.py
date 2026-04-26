import sys
from pathlib import Path
import pya

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.append(str(SCRIPT_DIR))

from cont import *

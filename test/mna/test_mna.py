from sstadex import mna

XSCHEM_RCFILE = "/opt/pdks/sky130A/libs.tech/xschem/xschemrc"
SPICE_DIR = "./spice/"
OUTPUT_DIR = "./output/"
XSCHEM_DIR = "./xschem/"

report, df, df2, A, X, Z, nodes = mna(XSCHEM_RCFILE, XSCHEM_DIR, SPICE_DIR, OUTPUT_DIR, "ota")

print(df)
print(A)
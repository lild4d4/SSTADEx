import sys
import os
import subprocess
import pandas as pd

XSCHEM_RCFILE = "/opt/pdks/sky130A/libs.tech/xschem/xschemrc"
SPICE_DIR = "./spice/"
OUTPUT_DIR = "./output/"
XSCHEM_DIR = "./xschem/"


def _simulate(sim_path, filename):
    with subprocess.Popen(
        ["ngspice", "-b", filename],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        cwd=sim_path,
        bufsize=1,
        start_new_session=True,
        universal_newlines=True,
    ) as spiceproc:
        pgroup = os.getpgid(spiceproc.pid)
        for line in spiceproc.stdout:
            print(line, end="")
            sys.stdout.flush()
            if "Simulation interrupted" in line:
                print("ngspice encountered an error. . . ending.")
                spiceproc.kill()

    spiceproc.stdout.close()
    return_code = spiceproc.wait()
    if return_code != 0:
        print("Error:  ngspice exited with non-zero status!")


def spice_sim(project, dict):
    spice_file = os.path.join(SPICE_DIR, project + ".spice")
    file = open(spice_file, "r")
    new_file = open(os.path.join(SPICE_DIR, "temp.spice"), "w")

    lines = file.readlines()

    dict_keys = dict.keys()

    results = []

    print(dict[list(dict_keys)[0]])
    for i in range(len(dict[list(dict_keys)[0]][0])):
        print("test numer: ", i)
        new_file = open(os.path.join(SPICE_DIR, "temp.spice"), "w")

        for idx, line in enumerate(lines):
            words = line.split()
            if len(words) == 0:
                print(line)
                new_file.write(line)
            elif words[0][0] == "X":
                print(words[0])
                try:
                    index = list(dict_keys).index(words[0])
                except ValueError:
                    index = -1
                if index != -1:
                    words[6] = "L=" + str(dict[words[0]][1][i]) + "u"
                    words[7] = "W=" + str(dict[words[0]][0][i]) + "u"
                    # if words[0][0] == "R" or words[0][0] == "C":
                    #    words[3] = str(dict[words[0]][i])
                    # elif words[0][0] == "G":
                    #    words[5] = str(dict[words[0]][i])
                    print(words)
                    words.append("\n")
                    line = " ".join(words)
                print(line)
                new_file.write(line)
            elif words[0][0] == "+":
                index = list(dict_keys).index(lines[idx - 1].split()[0])
                print("found +")
                if dict[lines[idx - 1].split()[0]][3] == "nfet":
                    print("in NFET")
                    print(str(dict[lines[idx - 1].split()[0]][2][i]))
                    words[15] = "mult=" + str(dict[lines[idx - 1].split()[0]][2][i])
                    words[16] = "m=" + str(dict[lines[idx - 1].split()[0]][2][i])
                else:
                    words[20] = "mult=" + str(dict[lines[idx - 1].split()[0]][2][i])
                    words[21] = "m=" + str(dict[lines[idx - 1].split()[0]][2][i])
                words.append("\n")
                line = " ".join(words)
                print(line)
                new_file.write(line)
            elif words[0][0] == "I":
                words[3] = str(dict[words[0]][i])
                print(words)
                words.append("\n")
                line = " ".join(words)
                print(line)
                new_file.write(line)
            elif words[0][0] == "V":
                try:
                    index = list(dict_keys).index(words[0])
                    words[3] = str(dict[words[0]][i])
                    print(words)
                    words.append("\n")
                    line = " ".join(words)
                except ValueError:
                    index = -1
                print(line)
                new_file.write(line)
            elif words[0][0] == "R":
                try:
                    index = list(dict_keys).index(words[0])
                    words[3] = str(dict[words[0]][i])
                    print(words)
                    words.append("\n")
                    line = " ".join(words)
                except ValueError:
                    index = -1
                print(line)
                new_file.write(line)
            else:
                print(line)
                new_file.write(line)

        new_file.close()
        _simulate(SPICE_DIR, "temp.spice")

        df = pd.read_csv(SPICE_DIR + "temp.csv", sep="\s+")
        results.append(df)

    return results

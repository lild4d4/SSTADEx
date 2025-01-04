import pandas as pd

def spice2ptxt(SPICE_DIR, OUTPUT_DIR, filename):

    spice_file = open(SPICE_DIR + filename + ".spice", "r")
    output_file = open(OUTPUT_DIR + filename + ".txt", "w")
    nodes = {}
    node_num = 1
    for line in spice_file:
        if line[0] != "*" and line[0] != ".":
            params = line.split(" ")
            if line[0] == "R" or line[0] == "C" or line[0] == "L":
                for net, index in [[params[1], 1], [params[2], 2]]:
                    if net not in nodes:
                        if net == "vss":
                            nodes[net] = 0
                        else:
                            nodes[net] = node_num
                            node_num = node_num + 1
                    params[index] = str(nodes[net])
                output_file.write(" ".join(params[:4]) + "\n")
            elif line[0] == "G":
                for net, index in [
                    [params[1], 1],
                    [params[2], 2],
                    [params[3], 3],
                    [params[4], 4],
                ]:
                    if net not in nodes:
                        if net == "vss":
                            nodes[net] = 0
                        else:
                            nodes[net] = node_num
                            node_num = node_num + 1
                    params[index] = str(nodes[net])
                output_file.write(" ".join(params[:6]))
            elif line[0] == "V" or line[0] == "I":
                for net, index in [[params[1], 1], [params[2], 2]]:
                    if net not in nodes:
                        if net == "vss":
                            nodes[net] = 0
                        else:
                            nodes[net] = node_num
                            node_num = node_num + 1
                    params[index] = str(nodes[net])
                output_file.write(" ".join(params[:4]))
            else:
                output_file.write(line)

    
    return pd.DataFrame([nodes])

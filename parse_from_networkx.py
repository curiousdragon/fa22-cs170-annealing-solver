import networkx as nx
import os
from os.path import join
import json
from pathlib import Path

def input_from_networkx_file(path: str, filename: str, input_path: str):
    info = []
    print(f"{filename}")

    with open(join(path, filename)) as fp:
        G = nx.node_link_graph(json.load(fp))
        info.append(str(G.number_of_nodes()))
        info.append(str(G.number_of_edges()))
        for u, v, d in G.edges(data=True):
            info.append(f"{u} {v} {d['weight']}")

    with open(join(input_path, filename), 'w') as f:
        for line in info:
            f.write(line)
            f.write('\n')

    print(f"{join(input_path, filename)} created")


def networkx_graph_from_input(path: str):
    with open(path) as f:
        lines = list(f)
        n = int(lines[0])
        e = int(lines[1])
        G = nx.Graph()
        G.add_nodes_from(range(n))
        for line in lines[2:]:
            i, j, w = line.split()
            G.add_edge(int(i), int(j))
            G.edges[int(i), int(j)]['weight'] = int(w)
        return G

def networkx_graph_from_output(path: str):
    with open(path) as f:
        lines = list(f)
        n = int(lines[0])
        e = int(lines[1])
        G = nx.Graph()
        G.add_nodes_from(range(n))
        for line in lines[2:2+e]:
            i, j, w = line.split()
            G.add_edge(int(i), int(j))
            G.edges[int(i), int(j)]['weight'] = int(w)
        for line in lines[2+e:]:
            i, team = line.split()
            # since teams should start at 1 apparently...
            G.nodes[int(i)]['team'] = int(team) + 1
        return G

def parse_all_networkx_inputs(path: str, input_path: str):
    for file in os.listdir(path):
        input_from_networkx_file(path, file, input_path)

parse_all_networkx_inputs("../latest-inputs", "inputs")

# /// script
# requires-python = ">=3.11"
# dependencies = [
#     "networkx",
#     "termcolor",
# ]
# ///

import networkx as nx
from termcolor import colored

def read_map(filename):
    with open(filename, 'r') as f:
        lines = [line.rstrip('\n') for line in f]
    grid = [list(line) for line in lines]
    return grid

def build_graph(grid):
    rows = len(grid)
    cols = len(grid[0]) if rows > 0 else 0
    G = nx.Graph()
    for y in range(rows):
        for x in range(cols):
            cell = grid[y][x]
            # Check if the cell is not an obstacle
            if cell not in {'#', '~'}:
                G.add_node((y, x))
                # Check all four adjacent cells
                for dy, dx in [(-1,0), (0,-1), (0,1), (1,0)]:
                    ny, nx_ = y + dy, x + dx
                    if 0 <= ny < rows and 0 <= nx_ < cols:
                        neighbor_cell = grid[ny][nx_]
                        if neighbor_cell not in {'#', '~'}:
                            G.add_edge((y, x), (ny, nx_))
    return G

def find_start_point(grid):
    # Find the only empty cell in the first row
    for x, cell in enumerate(grid[0]):
        if cell == '.':
            return (0, x)
    raise ValueError("Start point not found in the first row.")

def find_articulation_points_with_fruits(G, grid):
    # Find all articulation points
    articulation_points = list(nx.articulation_points(G))
    # Filter articulation points that contain fruits (cells not '.')
    fruit_articulation_points = [ap for ap in articulation_points if grid[ap[0]][ap[1]] != '.']
    return fruit_articulation_points

def compute_critical_articulation_points(G, start_point, fruit_articulation_points):
    # For each node, determine which articulation points are critical
    node_critical_aps = {}
    # Initialize the dictionary for all nodes
    for node in G.nodes():
        node_critical_aps[node] = set()
    # For each articulation point, remove it and find unreachable nodes
    for ap in fruit_articulation_points:
        G_copy = G.copy()
        G_copy.remove_node(ap)
        # Perform BFS from the start point
        if G_copy.has_node(start_point):
            reachable_nodes = nx.node_connected_component(G_copy, start_point)
        else:
            reachable_nodes = set()
        # Nodes not in reachable_nodes are disconnected when this AP is removed
        for node in G.nodes():
            if node not in reachable_nodes:
                node_critical_aps[node].add(ap)
    return node_critical_aps

def assign_colors(node_critical_aps, fruit_articulation_points):
    # Map combinations of critical articulation points to colors
    color_map = {}
    colors = ['red', 'green', 'yellow', 'blue', 'magenta', 'cyan']
    color_index = 0
    combination_to_color = {}
    for critical_set in set(frozenset(v) for v in node_critical_aps.values()):
        if critical_set not in combination_to_color:
            combination_to_color[critical_set] = colors[color_index % len(colors)]
            color_index += 1
    # Assign colors to nodes
    node_colors = {}
    for node, critical_set in node_critical_aps.items():
        node_colors[node] = combination_to_color[frozenset(critical_set)]
    return node_colors

def get_regions(G, node_colors):
    # Assign colors to nodes in the graph
    nx.set_node_attributes(G, node_colors, 'color')
    # Create a mapping from colors to subgraphs
    color_subgraphs = {}
    for color in set(node_colors.values()):
        # Get nodes of this color
        nodes_of_color = [node for node, data in G.nodes(data=True) if data['color'] == color]
        # Induce subgraph
        subgraph = G.subgraph(nodes_of_color)
        # Find connected components in this subgraph
        components = list(nx.connected_components(subgraph))
        color_subgraphs[color] = components
    return color_subgraphs

def print_regions_and_fruits(grid, color_subgraphs):
    # For each color, and each region (connected component), find fruits
    for color, components in color_subgraphs.items():
        for idx, component in enumerate(components):
            fruits_in_region = set()
            for node in component:
                y, x = node
                cell = grid[y][x]
                if cell not in {'.', '#', '~'}:
                    fruits_in_region.add(cell)
            # Print the fruits in this region
            fruit_list = ', '.join(sorted(fruits_in_region)) if fruits_in_region else 'None'
            print(f"Region {idx+1} of color '{color}' contains fruits: {fruit_list}")

def print_colored_map(grid, node_colors, start_point):
    rows = len(grid)
    cols = len(grid[0]) if rows > 0 else 0
    for y in range(rows):
        line = ''
        for x in range(cols):
            cell = grid[y][x]
            if cell in {'#', '~'}:
                # Obstacle cells
                line += cell
            else:
                node = (y, x)
                if node == start_point:
                    # Highlight the start point
                    line += colored(cell, 'white', attrs=['bold'])
                elif node in node_colors:
                    # Color the cell based on critical articulation points
                    color = node_colors[node]
                    line += colored(cell, color)
                else:
                    # Default color for nodes not in the graph (should not happen)
                    line += cell
        print(line)

def main():
    grid = read_map('src/part3.txt')
    G = build_graph(grid)
    start_point = find_start_point(grid)
    fruit_articulation_points = find_articulation_points_with_fruits(G, grid)
    
    print("Articulation Points containing fruits:")
    for ap in fruit_articulation_points:
        cell_content = grid[ap[0]][ap[1]]
        print(f"Position: {ap}, Cell Content: '{cell_content}'")
    
    node_critical_aps = compute_critical_articulation_points(G, start_point, fruit_articulation_points)
    node_colors = assign_colors(node_critical_aps, fruit_articulation_points)
    
    # Get regions (connected components) for each color
    color_subgraphs = get_regions(G, node_colors)
    
    # Print fruits in each region
    print("\nRegions and Fruits:")
    print_regions_and_fruits(grid, color_subgraphs)
    
    print("\nColored Map:")
    print_colored_map(grid, node_colors, start_point)

if __name__ == '__main__':
    main()

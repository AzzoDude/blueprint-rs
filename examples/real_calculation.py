import blueprint_rs
from blueprint_rs import Node, NodeValue, Connection, Graph, DataType

def test_real_calc():
    print("--- blueprint-rs Real Calculation Test ---")
    
    # 1. Create the Graph
    graph = Graph()
    
    # 2. Add Source Nodes (Values to be added)
    node_a = Node("Value_A", (0, 0))
    node_a.value = NodeValue.Int(75)
    
    node_b = Node("Value_B", (0, 100))
    node_b.value = NodeValue.Int(25)
    
    # 3. Add the "Arithmetic_Add" Node
    add_node = Node("Arithmetic_Add", (200, 50))
    
    # Register nodes in the graph
    graph.add_node(node_a)
    graph.add_node(node_b)
    graph.add_node(add_node)
    
    # 4. Create Connections
    # In a real app, you'd use port IDs, but our compute() logic 
    # currently looks at any connection to the target node's ID.
    conn1 = Connection(node_a.id, node_a.id, add_node.id, add_node.id)
    conn2 = Connection(node_b.id, node_b.id, add_node.id, add_node.id)
    
    graph.add_connection(conn1)
    graph.add_connection(conn2)
    
    print(f"Initial Add Node Value: {add_node.value}")
    
    # 5. PERFORM REAL CALCULATION
    print("\n[*] Triggering Graph Compute...")
    result = graph.compute(add_node.id)
    
    print(f"\nFinal Result: {result}")
    print(f"Logic: {node_a.value} + {node_b.value} = {result}")

if __name__ == "__main__":
    test_real_calc()

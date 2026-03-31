import blueprint_rs
from blueprint_rs import Node, DataType

def test_shape():
    print("--- blueprint-rs Node Shape Test ---")
    
    # 1. Create an "Add" node
    add_node = Node("Arithmetic_Add", (100, 100))
    
    # 2. Add 2 Inputs
    add_node.add_port("A", DataType.Float, False)
    add_node.add_port("B", DataType.Float, False)
    
    # 3. Add 1 Output
    add_node.add_port("Result", DataType.Float, True)
    
    # 4. Verify structural integrity
    print(f"Node Type: {add_node.name}")
    print(f"Inputs Found: {len(add_node.inputs)}")
    for p in add_node.inputs:
        print(f"  - Input: {p.name} (ID: {p.id})")
        
    print(f"Outputs Found: {len(add_node.outputs)}")
    for p in add_node.outputs:
        print(f"  - Output: {p.name} (ID: {p.id})")

if __name__ == "__main__":
    test_shape()

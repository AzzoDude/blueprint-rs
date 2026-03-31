import blueprint_rs
from blueprint_rs import Node, NodeValue

def test_math():
    print("--- blueprint-rs Math Test ---")
    
    # Setup
    n1 = Node("Value_1", (0, 0))
    n1.value = NodeValue.Int(50)
    
    n2 = Node("Value_2", (0, 0))
    n2.value = NodeValue.Float(2.5)
    
    # Addition (Mixed types Int + Float)
    res_add = n1.value + n2.value
    print(f"50 + 2.5 = {res_add}")
    
    # Multiplication
    res_mul = n1.value * n2.value
    print(f"50 * 2.5 = {res_mul}")
    
    # Division
    res_div = n1.value / n2.value
    print(f"50 / 2.5 = {res_div}")
    
    # String Concatenation
    s1 = NodeValue.String("Visual ")
    s2 = NodeValue.String("Scripting")
    print(f"String Add: '{s1 + s2}'")

if __name__ == "__main__":
    test_math()

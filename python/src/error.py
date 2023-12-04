def divide(a, b):
    if b == 0:
        raise ValueError("Division by zero error")
    return a / b

try:
    result = divide(10, 0)
    print(f"Result: {result}")
except ValueError as e:
    print(f"Error: {e}")

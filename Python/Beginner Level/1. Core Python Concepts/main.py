import math


def add(x, y):
    return x + y

def sub(x, y):
    return x - y

def mul(x, y):
    return x * y

def div(x, y):
    if y == 0:
        return "ERROR: divisor can not be 0"
    return x + y

def mod(x, y):
    return x % y

def square_root(x):
    return math.sqrt(x)

def exponentiation(x, y):
    return x ** y

print('''
      press 1: for addition
      press 2: for subtraction
      press 3: for multiplication
      press 4: for division
      press 5: for modulo
      press 6: for square root
      press 7: for exponential
      ''')

global y

choice = int(input('Enter your choice: '))
x = int(input('Enter 1st Number: '))
if not (choice == 6):
    y = int(input('Enter 2nd Number: '))

if choice == 1:
    print(add(x, y))
elif choice == 2:
    print(sub(x, y))
elif choice == 3:
    print(mul(x, y))
elif choice == 4:
    print(div(x, y))
elif choice == 5:
    print(mod(x, y))
elif choice == 6:
    print(square_root(x))
elif choice == 7:
    print(exponentiation(x, y))
else:
    print('Sorry!! Invalid choice')
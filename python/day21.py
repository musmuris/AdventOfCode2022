from pprint import pprint
testinput  = """root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"""

class Monkey:
    def __init__(self, name, val = None, left = "", right = "", op = ""):
        self.name = name
        self.val = val
        self.left = left
        self.right = right
        self.op = op


    def has_val(self):
        return self.val is not None
    
    def evaluate(self, l, r):        
        match self.op:
            case "+" : self.val = l + r
            case "*" : self.val = l * r
            case "-" : self.val = l - r
            case "/" : self.val = l / r
        print(f"{self.name}: {l} {self.op} {r} = {self.val}")
        return self.val

    def reverse_left(self, result, right):
        print(f"{self.name}: {result} {self.op} {right}")
        match self.op:
            case "+" : return result - right
            case "*" : return result / right
            case "-" : return result + right
            case "/" : return result * right        
    
    def reverse_right(self, result, left):
        print(f"{self.name}: {result} {self.op} {left}")
        match self.op:
            case "+" : return result - left
            case "*" : return result / left
            case "-" : return left - result 
            case "/" : return left / result


def inc(dict, key):
    if key in dict:
        dict[key] += 1
    else: 
        dict[key] = 1
    
def parse(lines):
    monkeys = {}
    for l in lines:
        name, rest = l.split(": ")
        if rest.isdigit() :
            monkeys[name] = Monkey( name, val = int(rest) )            
        else:
            left,op,right = rest.split(" ")
            monkeys[name] = Monkey( name, left = left, right = right, op = op )            
    return monkeys

def evaluate_as_tree(monkeys, current, humn_stack):
    if current.has_val():
        if current.name == "humn":
            humn_stack.append(current.name)
        print(f"{current.name} : {current.val}")
        return (current.val, current.name == "humn")
    left, had_humn_left = evaluate_as_tree(monkeys, monkeys[current.left], humn_stack)
    right, had_humn_right = evaluate_as_tree(monkeys, monkeys[current.right], humn_stack)
    current.evaluate(left, right)
    had_humn = had_humn_left or had_humn_right
    if had_humn:
        humn_stack.append(current.name)
    return (current.val, had_humn)

def unwind_stack(monkeys, stack):
    current = monkeys[stack.pop()]    
    next = stack[-1]
    if next == current.left:
        val = monkeys[current.right].val
    elif next == current.right:
        val = monkeys[current.left].val

    while len(stack) > 1:
        current = monkeys[stack.pop()]
        next = stack[-1]
        if next == current.left:
            val = current.reverse_left(val, monkeys[current.right].val)
        elif next == current.right:
            val = current.reverse_right(val, monkeys[current.left].val)
        else:
            raise Exception("Oh dear")
    print(val)

def run(input):
    m = parse(input)
    humn_stack = []
    left, had = evaluate_as_tree(m, m["root"], humn_stack)
    print(left)
    unwind_stack(m, humn_stack)

run(testinput.split("\n"))

input = open("python\\day21.txt", "r").read().splitlines()
run(input)

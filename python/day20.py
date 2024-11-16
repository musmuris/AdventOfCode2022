from itertools import cycle

input = [int(l) for l in open("python\\day20.txt", "r").readlines()]

def isneg(x):
    if x < 0 :
        return -1
    return 0

def mix(input, rounds, key):
    numbers = [n * key for n in input]    
    order = [n for n in range(len(numbers))]    
    for r in range(rounds):
        for (i,n) in enumerate(numbers):
            inx = order.index(i)
            del order[inx]
            new_inx = (inx + n ) % len(order)
            order.insert( new_inx, i)
            #print([numbers[x] for x in order])
    #print([numbers[x] for x in order])
    inx_zero = order.index(numbers.index(0))    
    sum = 0
    for m in [1000,2000,3000]:                
        sinx = (inx_zero + m ) % len(numbers)
        sum += numbers[order[sinx]]
    print(sum)

mix([1, 2, -3, 3, -2, 0, 4], 1, 1)
mix(input, 1, 1)
mix(input, 10, 811589153)
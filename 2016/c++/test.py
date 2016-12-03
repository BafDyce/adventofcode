def move(pos, instructions, numberpad):

    #dirty hack for positions
    for i in instructions:
        if i == "U":
            pos[0] = pos[0] - 1
            if pos[0] < 0:
                pos[0] = pos[0] + 1
        if i == "D":
            pos[0] = pos[0]+1
            if pos[0] > 2:
                pos[0] = pos[0] - 1
        if i == "R":
            pos[1] = pos[1]+1
            if pos[1] > 2:
                pos[1] = pos[1] - 1
        if i == "L":
            pos[1] = pos[1] - 1
            if pos[1] < 0:
                pos[1] = pos[1] + 1


    return numberpad[pos[0]][pos[1]]

def move2(pos, instructions, numberpad):
    # dirty hack for positions
    for i in instructions:
        print i

def getPos(number):
    if number == 1:
        return [0, 0]
    if number == 2:
        return [0, 1]
    if number == 3:
        return [0, 2]
    if number == 4:
        return [1, 0]
    if number == 5:
        return [1, 1]
    if number == 6:
        return [1, 2]
    if number == 7:
        return [2, 0]
    if number == 8:
        return [2, 1]
    if number == 9:
        return [2, 2]

    print "help!!! ", number

def getPos2(number):
    if number == 1:
        return [0, 2]
    if number == 2:
        return [1, 1]
    if number == 3:
        return [1, 2]
    if number == 4:
        return [1, 3]
    if number == 5:
        return [2, 0]
    if number == 6:
        return [2, 1]
    if number == 7:
        return [2, 2]
    if number == 8:
        return [2, 3]
    if number == 9:
        return [2, 4]
    if number == "A":
        return [3, 1]
    if number == "B":
        return [3, 2]
    if number == "C":
        return [3, 3]
    if number == "D":
        return [4, 2]

    print "help!!! ", number

#open directions
input = open("../_inputs/day02/puzzle3.input", "r")

instr = []

#read every line (one line is one instruction) into list
for line in input:
    instr.append(line)

#init numbpad
numberpad = [(1,2,3),(4,5,6),(7,8,9)]

#start at "5"
#first index up down
#second indes left right
pos = [1,1]

#write pin in here
pin = []

# main loop to get pin
for i in instr:
    pin.append(move(pos, i, numberpad))
    buff = move(pos, i, numberpad)
    pos = getPos(buff)

print pin

#------------ SECOND -----------------

#init numbpad
numberpad = [(0,0,1,0,0),(0,2,3,4,0),(5,6,7,8,9),(0,"A","B","C",0),(0,0,"D",0,0)]

#start at "5"
#first index up down
#second indes left right
pos = [2,0]
print numberpad[pos[0]][pos[1]]

#write pin in here
pin = []

for i in instr:
    pin.append(move(pos, i, numberpad))
    buff = move(pos, i, numberpad)
    pos = getPos2(buff)

print pin

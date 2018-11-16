set b 93
set c b
jnz a 2
jnz 1 5
mul b 100
sub b -100000
set c b
sub c -17000
set f 1
set d 2
set e 2
set g d
mul g e
sub g b
jnz g 2
set f 0
sub e -1
set g e
sub g b
jnz g -8
sub d -1
set g d
sub g b
jnz g -13
jnz f 2
sub h -1
set g b
sub g c
jnz g 2
jnz 1 3
sub b -17
jnz 1 -23


# debugging:

line 30: `jnz 1 3` must be executed

-> ln 29: `g` must be 0
-> ln 27: `g` must be `b` + `c`
    and `f` must be != 0
    and `g` must be 0 ?

```
b = 93
c = b
if(a != 0) jmp lbl_1
jmp lbl_2
lbl_1: b *= 100
b += 100000
c = b
c += 17000
lbl_2: f = 1
d = 2
lbl_7: e = 2
lbl_4: acc = d
acc *= e
acc -= b
if(acc != 0) jmp lbl_3
f = 0
lbl_3: e++
acc = e
acc -= b
if(acc != 0) jmp lbl_4
d++
acc = d
acc -= b
if(acc != 0) jmp lbl_7
if(f != 0) jmp lbl_5
h++
lbl_5: acc = b
acc -= c
if(acc != 0) jmp lbl_6
return h
lbl_6 b += 17
jnz 1 -23
```

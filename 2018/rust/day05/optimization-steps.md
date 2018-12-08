original:
```
real    2m1.160s
user    2m1.068s
sys     0m0.000s
```

use zwischenergebnis von part 1:
```
real    0m5.361s
user    0m5.352s
sys     0m0.007s
```

char.to_lowercase().next().unwrap() --> char.to_ascii_lowercase():
```
real    0m0.317s
user    0m0.317s
sys     0m0.000s
```

use std:;cmp::min():
```
real    0m0.309s
user    0m0.305s
sys     0m0.004s
```

dont build temporary strings:
```
real    0m0.230s
user    0m0.229s
sys     0m0.000s
```

String -> VecDeque<char>:
```
real    0m0.007s
user    0m0.000s
sys     0m0.007s
```

Expand the bit number of the parameters in the calculation formula to obtain the result with the expected bit number.
```
let a = u8::MAX;
let b = u16::MAX;
let result = calc!(a + b; u32);
assert_eq!(result, 65790u32);
```

# Multi-Compare
 
This crate provides a macro to compare multiple values in a single expression without
the need for the `&&` operator and without duplicating values.

## Example

```rust
use multi_compare::c;
 
fn main (){
    let mut a = 5;
    let mut b = 10;
    let mut c = 18;
     
    assert!(c!(a < b <= c));    
    assert!(c!(1 <= a < b <= c < 20));    
}
```

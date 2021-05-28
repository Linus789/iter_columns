# iter_columns
Iterate over columns easily.
Works with Vecs, Slices and Arrays.

## Examples
### Consistent column length
```rust
use iter_columns::prelude::*;

fn main() {
    let test_data = vec![
        vec![1, 2, 3], 
        vec![4, 5, 6],
    ];
    
    assert_eq!(test_data.into_iter().columns().collect::<Vec<_>>(), [
        [1, 4],
        [2, 5],
        [3, 6],
    ]);
}
```

### Inconsistent column length
```rust
use iter_columns::prelude::*;

fn main() {
    let test_data = vec![
        vec![1, 2],    // 2 columns
        vec![4, 5, 6], // 3 columns
    ];
    
    // you can also use iter() instead of into_iter()
    assert_eq!(test_data.iter().columns().collect::<Vec<_>>(), [
        vec![&1, &4],
        vec![&2, &5],
        vec![&6],
    ]);
}
```

### Alternative for inconsistent column length
```rust
use iter_columns::prelude::*;

fn main() {
    let test_data = vec![
        vec![1, 2],
        vec![4, 5, 6],
    ];
    
    // use columns_options() instead of columns()
    assert_eq!(test_data.into_iter().columns_options().collect::<Vec<_>>(), [
        vec![Some(1), Some(4)],
        vec![Some(2), Some(5)],
        vec![None, Some(6)],
    ]);
}
```
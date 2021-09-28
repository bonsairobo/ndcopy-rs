# ndcopy

Fast N-dimensional array memcpy.

Speed is achieved by copying slices row-by-row. Rust code is much faster at copying slices than trying to index
N-dimensional coordinates for every value index.

## Example Code

```rust
use ndcopy::ndshape::{ConstShape, ConstShape3u32};
use ndcopy::copy3;

type SrcShape = ConstShape3u32<100, 100, 100>;
type DstShape = ConstShape3u32<50, 50, 50>;
let src = [1u8; SrcShape::SIZE as usize];
let mut dst = [0u8; DstShape::SIZE as usize];

let copy_shape = [20; 3];
let src_min = [1, 2, 3];
let dst_min = [2, 3, 4];
copy3(
    copy_shape,
    &src,
    &SrcShape {},
    src_min,
    &mut dst,
    &DstShape {},
    dst_min,
);
```

License: MIT OR Apache-2.0

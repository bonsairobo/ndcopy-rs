//! Fast N-dimensional array memcpy.
//!
//! Speed is achieved by copying slices row-by-row. Rust code is much faster at copying slices than trying to index
//! N-dimensional coordinates for every value index.
//!
//! # Example Code
//!
//! ```
//! use ndcopy::ndshape::{ConstShape, ConstShape3u32};
//! use ndcopy::copy3;
//!
//! type SrcShape = ConstShape3u32<100, 100, 100>;
//! type DstShape = ConstShape3u32<50, 50, 50>;
//! let src = [1u8; SrcShape::SIZE as usize];
//! let mut dst = [0u8; DstShape::SIZE as usize];
//!
//! let copy_shape = [20; 3];
//! let src_min = [1, 2, 3];
//! let dst_min = [2, 3, 4];
//! copy3(
//!     copy_shape,
//!     &src,
//!     &SrcShape {},
//!     src_min,
//!     &mut dst,
//!     &DstShape {},
//!     dst_min,
//! );
//! ```

pub use ndshape;

use ndshape::Shape;

/// Copy 2-dimensional data from `src` to `dst`.
///
/// - `copy_shape`: Dimensions of the extent to be copied.
/// - `src`: The source slice.
/// - `src_shape`: A `Shape<u32, 2>` for the entire `src` slice.
/// - `src_start`: The starting 2D offset to copy from `src`.
/// - `dst`: The destination slice.
/// - `dst_shape`: A `Shape<u32, 2>` for the entire `dst` slice.
/// - `dst_start`: The starting 2D offset to copy into `dst`.
#[inline]
pub fn copy2<T, Src, Dst>(
    copy_shape: [u32; 2],
    src: &[T],
    src_shape: &Src,
    src_start: [u32; 2],
    dst: &mut [T],
    dst_shape: &Dst,
    dst_start: [u32; 2],
) where
    T: Clone,
    Src: Shape<u32, 2>,
    Dst: Shape<u32, 2>,
{
    let row_length = copy_shape[0];

    let mut src_y = src_start[1];
    let mut dst_y = dst_start[1];
    for _ in 0..copy_shape[1] {
        let src_row_start = src_shape.linearize([src_start[0], src_y]) as usize;
        let src_row_end = src_row_start + row_length as usize;

        let dst_row_start = dst_shape.linearize([dst_start[0], dst_y]) as usize;
        let dst_row_end = dst_row_start + row_length as usize;

        dst[dst_row_start..dst_row_end].clone_from_slice(&src[src_row_start..src_row_end]);

        src_y += 1;
        dst_y += 1;
    }
}

#[test]
fn test_copy2() {
    use ndshape::ConstShape2u32;

    let src_shape = ConstShape2u32::<10, 11>;
    const SRC_SIZE: usize = 10 * 11;
    let src = [1; SRC_SIZE];
    let dst_shape = ConstShape2u32::<11, 12>;
    const DST_SIZE: usize = 11 * 12;
    let mut dst = [0; DST_SIZE];

    copy2(
        [2, 3],
        &src,
        &src_shape,
        [3, 4],
        &mut dst,
        &dst_shape,
        [4, 5],
    );

    for y in 5..5 + 3 {
        for x in 4..4 + 2 {
            let i = dst_shape.linearize([x, y]) as usize;
            assert_eq!(1, dst[i]);
            dst[i] = 0;
        }
    }
    for i in 0..DST_SIZE {
        assert_eq!(dst[i], 0);
    }
}

/// Fill a 2-dimensional extent of `dst` with `value`.
///
/// - `fill_shape`: Dimensions of the extent to be copied.
/// - `value`: The value to write.
/// - `dst`: The destination slice.
/// - `dst_shape`: A `Shape<u32, 2>` for the entire `dst` slice.
/// - `dst_start`: The starting 2D offset to copy into `dst`.
#[inline]
pub fn fill2<T, Dst>(
    fill_shape: [u32; 2],
    value: T,
    dst: &mut [T],
    dst_shape: &Dst,
    dst_start: [u32; 2],
) where
    T: Clone,
    Dst: Shape<u32, 2>,
{
    let row_length = fill_shape[0];

    let mut dst_y = dst_start[1];
    for _ in 0..fill_shape[1] {
        let dst_row_start = dst_shape.linearize([dst_start[0], dst_y]) as usize;
        let dst_row_end = dst_row_start + row_length as usize;

        dst[dst_row_start..dst_row_end].fill(value.clone());

        dst_y += 1;
    }
}

#[test]
fn test_fill2() {
    use ndshape::ConstShape2u32;

    let dst_shape = ConstShape2u32::<11, 12>;
    const DST_SIZE: usize = 11 * 12;
    let mut dst = [0; DST_SIZE];

    fill2([2, 3], 1, &mut dst, &dst_shape, [4, 5]);

    for y in 5..5 + 3 {
        for x in 4..4 + 2 {
            let i = dst_shape.linearize([x, y]) as usize;
            assert_eq!(1, dst[i]);
            dst[i] = 0;
        }
    }
    for i in 0..DST_SIZE {
        assert_eq!(dst[i], 0);
    }
}

/// Copy 3-dimensional data from `src` to `dst`.
///
/// - `copy_shape`: Dimensions of the extent to be copied.
/// - `src`: The source slice.
/// - `src_shape`: A `Shape<u32, 3>` for the entire `src` slice.
/// - `src_start`: The starting 3D offset to copy from `src`.
/// - `dst`: The destination slice.
/// - `dst_shape`: A `Shape<u32, 3>` for the entire `dst` slice.
/// - `dst_start`: The starting 3D offset to copy into `dst`.
#[inline]
pub fn copy3<T, Src, Dst>(
    copy_shape: [u32; 3],
    src: &[T],
    src_shape: &Src,
    src_start: [u32; 3],
    dst: &mut [T],
    dst_shape: &Dst,
    dst_start: [u32; 3],
) where
    T: Clone,
    Src: Shape<u32, 3>,
    Dst: Shape<u32, 3>,
{
    let row_length = copy_shape[0];

    let mut src_z = src_start[2];
    let mut dst_z = dst_start[2];
    for _ in 0..copy_shape[2] {
        let mut src_y = src_start[1];
        let mut dst_y = dst_start[1];
        for _ in 0..copy_shape[1] {
            let src_row_start = src_shape.linearize([src_start[0], src_y, src_z]) as usize;
            let src_row_end = src_row_start + row_length as usize;

            let dst_row_start = dst_shape.linearize([dst_start[0], dst_y, dst_z]) as usize;
            let dst_row_end = dst_row_start + row_length as usize;

            dst[dst_row_start..dst_row_end].clone_from_slice(&src[src_row_start..src_row_end]);

            src_y += 1;
            dst_y += 1;
        }
        src_z += 1;
        dst_z += 1;
    }
}

#[test]
fn test_copy3() {
    use ndshape::ConstShape3u32;

    let src_shape = ConstShape3u32::<10, 11, 12>;
    const SRC_SIZE: usize = 10 * 11 * 12;
    let src = [1; SRC_SIZE];

    let dst_shape = ConstShape3u32::<11, 12, 13>;
    const DST_SIZE: usize = 11 * 12 * 13;
    let mut dst = [0; DST_SIZE];

    copy3(
        [2, 3, 4],
        &src,
        &src_shape,
        [3, 4, 5],
        &mut dst,
        &dst_shape,
        [4, 5, 6],
    );

    for z in 6..6 + 4 {
        for y in 5..5 + 3 {
            for x in 4..4 + 2 {
                let i = dst_shape.linearize([x, y, z]) as usize;
                assert_eq!(1, dst[i]);
                dst[i] = 0;
            }
        }
    }
    for i in 0..DST_SIZE {
        assert_eq!(dst[i], 0);
    }
}

/// Fill a 3-dimensional extent of `dst` with `value`.
///
/// - `fill_shape`: Dimensions of the extent to be copied.
/// - `value`: The value to write.
/// - `dst`: The destination slice.
/// - `dst_shape`: A `Shape<u32, 3>` for the entire `dst` slice.
/// - `dst_start`: The starting 3D offset to copy into `dst`.
#[inline]
pub fn fill3<T, Dst>(
    fill_shape: [u32; 3],
    value: T,
    dst: &mut [T],
    dst_shape: &Dst,
    dst_start: [u32; 3],
) where
    T: Clone,
    Dst: Shape<u32, 3>,
{
    let row_length = fill_shape[0];

    let mut dst_z = dst_start[2];
    for _ in 0..fill_shape[2] {
        let mut dst_y = dst_start[1];
        for _ in 0..fill_shape[1] {
            let dst_row_start = dst_shape.linearize([dst_start[0], dst_y, dst_z]) as usize;
            let dst_row_end = dst_row_start + row_length as usize;

            dst[dst_row_start..dst_row_end].fill(value.clone());

            dst_y += 1;
        }
        dst_z += 1;
    }
}

#[test]
fn test_fill3() {
    use ndshape::ConstShape3u32;

    let dst_shape = ConstShape3u32::<11, 12, 13>;
    const DST_SIZE: usize = 11 * 12 * 13;
    let mut dst = [0; DST_SIZE];

    fill3([2, 3, 4], 1, &mut dst, &dst_shape, [4, 5, 6]);

    for z in 6..6 + 4 {
        for y in 5..5 + 3 {
            for x in 4..4 + 2 {
                let i = dst_shape.linearize([x, y, z]) as usize;
                assert_eq!(1, dst[i]);
                dst[i] = 0;
            }
        }
    }
    for i in 0..DST_SIZE {
        assert_eq!(dst[i], 0);
    }
}

/// Copy 4-dimensional data from `src` to `dst`.
///
/// - `copy_shape`: Dimensions of the extent to be copied.
/// - `src`: The source slice.
/// - `src_shape`: A `Shape<u32, 4>` for the entire `src` slice.
/// - `src_start`: The starting 4D offset to copy from `src`.
/// - `dst`: The destination slice.
/// - `dst_shape`: A `Shape<u32, 4>` for the entire `dst` slice.
/// - `dst_start`: The starting 4D offset to copy into `dst`.
#[inline]
pub fn copy4<T, Src, Dst>(
    copy_shape: [u32; 4],
    src: &[T],
    src_shape: &Src,
    src_start: [u32; 4],
    dst: &mut [T],
    dst_shape: &Dst,
    dst_start: [u32; 4],
) where
    T: Clone,
    Src: Shape<u32, 4>,
    Dst: Shape<u32, 4>,
{
    let row_length = copy_shape[0];

    let mut src_w = src_start[3];
    let mut dst_w = dst_start[3];
    for _ in 0..copy_shape[3] {
        let mut src_z = src_start[2];
        let mut dst_z = dst_start[2];
        for _ in 0..copy_shape[2] {
            let mut src_y = src_start[1];
            let mut dst_y = dst_start[1];
            for _ in 0..copy_shape[1] {
                let src_row_start =
                    src_shape.linearize([src_start[0], src_y, src_z, src_w]) as usize;
                let src_row_end = src_row_start + row_length as usize;

                let dst_row_start =
                    dst_shape.linearize([dst_start[0], dst_y, dst_z, dst_w]) as usize;
                let dst_row_end = dst_row_start + row_length as usize;

                dst[dst_row_start..dst_row_end].clone_from_slice(&src[src_row_start..src_row_end]);

                src_y += 1;
                dst_y += 1;
            }
            src_z += 1;
            dst_z += 1;
        }
        src_w += 1;
        dst_w += 1;
    }
}

#[test]
fn test_copy4() {
    use ndshape::ConstShape4u32;

    let src_shape = ConstShape4u32::<10, 11, 12, 13>;
    const SRC_SIZE: usize = 10 * 11 * 12 * 13;
    let src = [1; SRC_SIZE];

    let dst_shape = ConstShape4u32::<11, 12, 13, 14>;
    const DST_SIZE: usize = 11 * 12 * 13 * 14;
    let mut dst = [0; DST_SIZE];

    copy4(
        [2, 3, 4, 5],
        &src,
        &src_shape,
        [3, 4, 5, 6],
        &mut dst,
        &dst_shape,
        [4, 5, 6, 7],
    );

    for w in 7..7 + 5 {
        for z in 6..6 + 4 {
            for y in 5..5 + 3 {
                for x in 4..4 + 2 {
                    let i = dst_shape.linearize([x, y, z, w]) as usize;
                    assert_eq!(1, dst[i]);
                    dst[i] = 0;
                }
            }
        }
    }
    for i in 0..DST_SIZE {
        assert_eq!(dst[i], 0);
    }
}

/// Fill a 4-dimensional extent of `dst` with `value`.
///
/// - `fill_shape`: Dimensions of the extent to be copied.
/// - `value`: The value to write.
/// - `dst`: The destination slice.
/// - `dst_shape`: A `Shape<u32, 4>` for the entire `dst` slice.
/// - `dst_start`: The starting 4D offset to copy into `dst`.
#[inline]
pub fn fill4<T, Dst>(
    fill_shape: [u32; 4],
    value: T,
    dst: &mut [T],
    dst_shape: &Dst,
    dst_start: [u32; 4],
) where
    T: Clone,
    Dst: Shape<u32, 4>,
{
    let row_length = fill_shape[0];

    let mut dst_w = dst_start[3];
    for _ in 0..fill_shape[3] {
        let mut dst_z = dst_start[2];
        for _ in 0..fill_shape[2] {
            let mut dst_y = dst_start[1];
            for _ in 0..fill_shape[1] {
                let dst_row_start =
                    dst_shape.linearize([dst_start[0], dst_y, dst_z, dst_w]) as usize;
                let dst_row_end = dst_row_start + row_length as usize;

                dst[dst_row_start..dst_row_end].fill(value.clone());

                dst_y += 1;
            }
            dst_z += 1;
        }
        dst_w += 1;
    }
}

#[test]
fn test_fill4() {
    use ndshape::ConstShape4u32;

    let dst_shape = ConstShape4u32::<11, 12, 13, 14>;
    const DST_SIZE: usize = 11 * 12 * 13 * 14;
    let mut dst = [0; DST_SIZE];

    fill4([2, 3, 4, 5], 1, &mut dst, &dst_shape, [4, 5, 6, 7]);

    for w in 7..7 + 5 {
        for z in 6..6 + 4 {
            for y in 5..5 + 3 {
                for x in 4..4 + 2 {
                    let i = dst_shape.linearize([x, y, z, w]) as usize;
                    assert_eq!(1, dst[i]);
                    dst[i] = 0;
                }
            }
        }
    }
    for i in 0..DST_SIZE {
        assert_eq!(dst[i], 0);
    }
}

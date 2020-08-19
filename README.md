[![github]](https://github.com/usagi/dimension_shiftable_buffer)&ensp;[![crates-io]](https://crates.io/crates/dimension_shiftable_buffer)&ensp;[![docs-rs]](https://docs.rs/dimension_shiftable_buffer)<br>
[![Build Status](https://travis-ci.org/usagi/dimension_shiftable_buffer.svg?branch=master)](https://travis-ci.org/usagi/dimension_shiftable_buffer)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K

# Dimension shiftable buffer

The `DimensionShiftableBuffer` by this crate presents a dimension shiftable buffer feature.

## What's the "dimension shiftable buffer"?

- First, a dimension "un-shiftable" buffer is a common buffer types such as:
  - `[T;3]` = One datum, 3-fixed-dimension, buffer is allocated a single heap, elements placed on a sequential memory addresses.
  - `[[T;3]]` = 0..* data, 3-fixed-dimension, buffer is allocated a single heap, elements placed on a sequential memory addresses.
  - `Vec<T>` = 0..* data, 1-fiexed-dimension, buffer is allocated a single heap, elements placed on a sequential memory addresses.
  - `Vec<[T;3]>` = 0..* data, 3-fixed-dimension, buffer is allocated a single heap, elements placed on a sequential memory addresses.
  - `Vec<Vec<T>>` = 0..* data, 1..*-any-dimension, buffer is allocated a multiple heap, elements placed on a dispersed memory addresses.
- Then, a dimension "shiftable" buffer is:
  - `DimensionShiftableBuffer<T>` = 0..* data, N-shiftable-dimension, buffer is allocated a single heap, elements places on a sequential memory addresses.
    - Has one buffer heap.
    - Can access as N=1..* dimension view such as like a `[&[T;3]]`, `[&mut [T;3]]`, `[&[T;127]]`.
    - With only safe implementation.

## Example

```toml
[dependencies]
dimension_shiftable_buffer = "*"
```

### Example-1

A simple usages.

```rust
// make a 2d-empty DimensionShiftableBuffer
let mut dsb = DimensionShiftableBuffer::<u8>::new(vec![], 2).unwrap();
// push a 2d-datum
dsb.push(&[0u8, 1]).unwrap();
// push a 2d-datum
dsb.push(&[2u8, 3]).unwrap();
// append a 2d-datum sequence
dsb.append(&[4u8, 5, 6, 7, 8, 9, 10, 11]).unwrap();
for index in 0..dsb.len().unwrap()
{
 // get a 2d slice
 assert_eq!(dsb.get(index).unwrap(), &[index as u8 * 2, index as u8 * 2 + 1]);
}
// shift dimension to 3 from 2
dsb.shift_dimension(3).unwrap();
// push a 3d-datum
dsb.push(&[12u8, 13, 14]).unwrap();
// get a 3d-datum
assert_eq!(dsb.get(0).unwrap(), &[0u8, 1, 2]);
assert_eq!(dsb.get(1).unwrap(), &[3u8, 4, 5]);
assert_eq!(dsb.get(2).unwrap(), &[6u8, 7, 8]);
assert_eq!(dsb.get(3).unwrap(), &[9u8, 10, 11]);
assert_eq!(dsb.get(4).unwrap(), &[12u8, 13, 14]);
// get a linear slice
let linear_slice = dsb.as_slice();
assert_eq!(linear_slice, &[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
```

### Example-2

With an error handling.

```rust
use dimension_shiftable_buffer::DimensionShiftableBuffer

 // Index of 1D =>           0      |1   |2   |3   |4   |5   |6   |7   |8   |
 // Index of 2D =>           0,0     0,1 |1,0  1,1 |2,0  2,1 |3,0  3,1 |4,0  4,1 |
 // Index of 3D =>           0,0     0,1  0,2 |1,0  1,1  1,2 |2,0  2,1  2,2 |
 // Index of 7D =>           0,0     0,1  0,2  0,3  0,4  0,5  0,6 |1,0  1,1  1,2  1,3  1,4  1,5  1,6 |
 let entity = vec![1.1f32, 1.2, 1.3, 2.1, 2.2, 2.3, 3.1, 3.2, 3.3];
 let dimension = 1;

 let mut dsb = DimensionShiftableBuffer::<f32>::new(entity, dimension).unwrap();

 let d1_i3 = dsb.get(3).unwrap();
 assert_eq!(d1_i3, [2.1f32]);

 dsb.shift_dimension(3).unwrap();

 let d3_i2 = dsb.get(2).unwrap();
 assert_eq!(d3_i2, [3.1f32, 3.2, 3.3]);

 match dsb.shift_dimension(2)
 {
  Ok(_) => panic!("Detect an unexpected false-ok."),
  Err(_) => eprintln!("Expected err.")
 }

 dsb.shift_dimension_truncate(2).unwrap();

 let d2_i1 = dsb.get(1).unwrap();
 assert_eq!(d2_i1, [1.3f32, 2.1]);

 dsb.shift_dimension_padding(7, std::f32::NAN).unwrap();
 let d7_i1 = dsb.get(1).unwrap();
 assert_eq!(
  // rounding, just easily
  format!("{:?}", d7_i1),
  format!("{:?}", &[
   // [0] of 7-d
   3.2f32,
   // [1] of 7-d, truncated at .shift_dimension.truncate(2) -> padding at .shift_dimension_padding(7)
   std::f32::NAN,
   // [2] of 7-d, padding at .shift_dimension_padding(7)
   std::f32::NAN,
   std::f32::NAN,
   std::f32::NAN,
   std::f32::NAN,
   std::f32::NAN
  ])
 )
```

### Example-3

Additional feature="csv" (default).; CSV=Character-Separated-Values, also known as DSV or TSV.

```
let delimiter = " ";
let source = "0.0 1.0 2.0 3.0 4.0 5.0 6.0 7.0 8.0 9.0 10.0 11.0 12.0 13.0 14.0 15.0 16.0 17.0";
let dimension = 3;
// construct
let mut dsb = DimensionShiftableBuffer::<f32>::from_csv(source, dimension, delimiter).unwrap();
// modify-1
dsb.for_each(|v| *v += 0.1);
// modify-2
for n in 0..dsb.len().unwrap()
{
 for v in dsb.get_mut(n).unwrap()
 {
  *v *= *v;
 }
}
// show
eprintln!("{:?}", dsb);
// enumerate
for i in 0..18
// maybe, 0..5 will ok, 6..18 will err
{
 match dsb.get(i)
 {
  Ok(dimensional_elements) =>
  {
   for (i_sub, actual) in dimensional_elements.iter().enumerate()
   {
    let i_flatten = i * dimension as usize + i_sub;
    let expected = (i_flatten as f32 + 0.1) * (i_flatten as f32 + 0.1);
    assert_eq!(actual, &expected);
   }
  },
  Err(e) =>
  {
   match e
   {
    DimensionShiftableBufferError::OutOfRange {
     limit: _,
     requested: _
    } => (),
    _ => panic!("unexpected err: {:?}", e)
   }
  },
 }
}
println!("<<print CSV, as a sequential slice>>\n{}", dsb.to_csv(", "));
println!(
 "<<print CSV, as a dimmensional slices>>\n{}",
 dsb.to_csv_dimensional(", ", "\n").unwrap()
);
```

```sh
<<print CSV, as a sequential slice>>
0.010000001, 1.21, 4.4099994, 9.61, 16.81, 26.009998, 37.21, 50.41, 65.61001, 82.810005, 102.01001, 123.21001, 146.41, 171.61002, 198.81001, 228.01001, 259.21002, 292.41
<<print CSV, as a dimmensional slices>>
0.010000001, 1.21, 4.4099994
9.61, 16.81, 26.009998
37.21, 50.41, 65.61001
82.810005, 102.01001, 123.21001
146.41, 171.61002, 198.81001
228.01001, 259.21002, 292.41
```

## License

- [MIT](LICENSE.md)

## Author

- [USAGI.NETWORK / Usagi Ito](https://github.com/usagi/)

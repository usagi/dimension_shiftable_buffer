use dimension_shiftable_buffer::DimensionShiftableBuffer;

fn example_u8()
{
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
}

fn main()
{
 example_u8();
}
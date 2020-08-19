use dimension_shiftable_buffer::DimensionShiftableBuffer;

fn example_f32()
{
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
}

fn main()
{
 example_f32();
}
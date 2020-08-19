use dimension_shiftable_buffer::{
 Csv,
 DimensionShiftableBuffer,
 DimensionShiftableBufferError
};

fn example_csv()
{
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
}

fn main()
{
 example_csv();
}

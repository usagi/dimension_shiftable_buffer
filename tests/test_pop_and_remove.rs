use dimension_shiftable_buffer::DimensionShiftableBuffer;

#[test]
fn test_pop_and_remove()
{
 let mut dsb = DimensionShiftableBuffer::<f64>::new(vec![], 3).unwrap();

 dsb.push(&[1.0, 2.0, 3.0]).unwrap();
 dsb.push(&[4.0, 5.0, 6.0]).unwrap();
 dsb.push(&[-1.0, -2.0, -3.0]).unwrap();
 assert_eq!(dsb.len().unwrap(), 3);

 let popped = dsb.pop().unwrap();
 assert_eq!(popped, [-1.0, -2.0, -3.0]);
 assert_eq!(dsb.len().unwrap(), 2);

 dsb.push(&popped[..]).unwrap();
 let removed = dsb.remove(1).unwrap();
 assert_eq!(removed, [4.0, 5.0, 6.0]);
 assert_eq!(dsb.len().unwrap(), 2);
}

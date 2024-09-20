/// This function must handle the sum of integers with different data types, however
/// it should not mix between types, since Rust enforces strict-type safety.
/// To handle the sum of any data-type we use generics and traits.
/// 
/// 1) To define a generic we must use: <T> for convention T stands for Type, but we can
///    use any letter of word you seen fit. Ex: <G> <gen>, this behaviour is similar to
///    size_t usage in C Language.
/// 2) To define a trait, we must define how the <T>'s will interact with each other, in
///    this example they will perform together a 'Addition', this explain the usage of 
///    <T: Add<Output = T>>, this will tell the functions that we expect a result of the
///    same type.

use std::ops::Add;

fn	sum<T: Add<Output = T>>(a: T, b: T) -> T {
	return a + b;
}

/*fn	main()
{
	let	result: i64;
	result = sum(4_000_000_000, 9_000_000_000);
	println!("{}", result);
}*/

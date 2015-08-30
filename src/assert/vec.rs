use std::collections::HashSet;
use std::fmt::{Debug, Write};

pub struct AssertVec<T>(Vec<T>);

impl<T> AssertVec<T> {
	pub fn new(v : Vec<T>) -> AssertVec<T> {
		AssertVec(v)
	}
}

impl<T: Eq + Debug> AssertVec<T> {
	pub fn contains_only(&self, expected : Vec<T>) {
		let mut exp_leftovers : Vec<_>
			= (0..expected.len()).collect();
		
		let mut self_leftovers = HashSet::new();
		for (self_idx, value) in self.0.iter().enumerate() {
			let i = exp_leftovers.iter()
				.enumerate()
				.filter_map(|(i, &exp_idx)|
					if expected[exp_idx] == *value {
						Some(i)
					} else {
						None
					})
				.next();
			
			match i {
				Some(i) => { exp_leftovers.swap_remove(i); },
				None => { self_leftovers.insert(self_idx); }
			}
		}
		
		let mut error_message = String::new();
		let mut assertion_failed = false;
		if !self_leftovers.is_empty() {
			write!(&mut error_message, "Unexpected values: ").unwrap();
			AssertVec::write_bad_values(
				&mut error_message,
				&self.0,
				self_leftovers);
			assertion_failed = true;
		}
		if !exp_leftovers.is_empty() {
			write!(&mut error_message, "Missing expected values: ").unwrap();
			AssertVec::write_bad_values(
				&mut error_message,
				&expected,
				exp_leftovers.into_iter().collect());
			assertion_failed = true;
		}
		
		if assertion_failed {
			panic!("Vectors contain different values:\n{}", error_message);
		}
	}
	
	fn write_bad_values
	(str_buf : &mut String, values : &Vec<T>, bad_indices : HashSet<usize>) {
		write!(str_buf, "[").unwrap();;
		let mut iter = values.into_iter().enumerate();
		let (i, value) = iter.next().unwrap();
		if bad_indices.contains(&i) {
			write!(str_buf, "{:?}", value).unwrap();;
		} else {
			write!(str_buf, "_").unwrap();;
		}
		for (i, value) in iter {
			if bad_indices.contains(&i) {
				write!(str_buf, ", {:?}", value).unwrap();;
			} else {
				write!(str_buf, ", _").unwrap();;
			}
		}
		writeln!(str_buf, "]").unwrap();;
	}
}

#[cfg(test)]
mod test {
	use super::*;
	
	#[test]
	fn contains_only_empty_vectors() {
		let sut = AssertVec::<u32>::new(Vec::new());
		let expected = Vec::new();
		sut.contains_only(expected);
	}

	#[test]
	fn contains_only_single_equal_element() {
		let sut = AssertVec::<u32>::new(vec![0]);
		let expected = vec![0];
		sut.contains_only(expected);
	}

	#[test]
	fn contains_only_multiple_equal_elements_same_order() {
		let sut = AssertVec::<u32>::new(vec![5, 8, 2]);
		let expected = vec![5, 8, 2];
		sut.contains_only(expected);
	}

	#[test]
	fn contains_only_multiple_equal_elements_different_order() {
		let sut = AssertVec::<u32>::new(vec![1, 2, 3, 4, 5]);
		let expected = vec![2, 3, 5, 1, 4];
		sut.contains_only(expected);
	}

	#[test]
	fn contains_only_equal_duplicate_elements() {
		let sut = AssertVec::<u32>::new(vec![1, 3, 2, 2, 1, 3]);
		let expected = vec![1, 1, 2, 2, 3, 3];
		sut.contains_only(expected);
	}

	#[test]
	#[should_panic(expected = "Vectors contain different values:\nMissing expected values: [0]\n")]
	fn contains_only_empty_asserter() {
		let sut = AssertVec::<u32>::new(Vec::new());
		let expected = vec![0];
		sut.contains_only(expected);
	}

	#[test]
	#[should_panic(expected = "Vectors contain different values:\nUnexpected values: [0]\n")]
	fn contains_only_empty_expected() {
		let sut = AssertVec::<u32>::new(vec![0]);
		let expected = Vec::new();
		sut.contains_only(expected);
	}

	#[test]
	#[should_panic(expected = "Vectors contain different values:\nUnexpected values: [5]\nMissing expected values: [0]\n")]
	fn contains_only_single_unequal_element() {
		let sut = AssertVec::<u32>::new(vec![5]);
		let expected = vec![0];
		sut.contains_only(expected);
	}

	#[test]
	#[should_panic(expected = "Vectors contain different values:\nUnexpected values: [_, _, 5]\nMissing expected values: [_, _, 2]\n")]
	fn contains_only_unequal_element_at_end() {
		let sut = AssertVec::<u32>::new(vec![6, 4, 5]);
		let expected = vec![6, 4, 2];
		sut.contains_only(expected);
	}

	#[test]
	#[should_panic(expected = "Vectors contain different values:\nUnexpected values: [1, 2, 3, 4, 5]\nMissing expected values: [6, 7, 8, 9, 10]\n")]
	fn contains_only_multiple_unequal_elements() {
		let sut = AssertVec::<u32>::new(vec![1, 2, 3, 4, 5]);
		let expected = vec![6, 7, 8, 9, 10];
		sut.contains_only(expected);
	}

	#[test]
	#[should_panic(expected = "Vectors contain different values:\nUnexpected values: [_, _, _, _, 5]\n")]
	fn contains_only_different_length_vecs() {
		let sut = AssertVec::<u32>::new(vec![1, 2, 3, 4, 5]);
		let expected = vec![1, 2, 3, 4];
		sut.contains_only(expected);
	}

	#[test]
	#[should_panic(expected = "Vectors contain different values:\nUnexpected values: [_, _, 2, 1, _]\n")]
	fn contains_only_unequal_duplicate_elements() {
		let sut = AssertVec::<u32>::new(vec![1, 2, 2, 1, 3]);
		let expected = vec![1, 2, 3];
		sut.contains_only(expected);
	}
}

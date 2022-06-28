pub fn selection_sort(arr: &mut [i32]) {
	let length = arr.len();

	if length <= 1 {
		return;
	}

	for i in (1..length).rev() {
		let mut max_index = i;
		for j in 0..i {
			if arr[j] > arr[max_index] {
				max_index = j;
			}
		}
		arr.swap(max_index, i);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_fixed_array() {
		let mut arr = [2, 1, 3, 6, 7, 4, 5];
		selection_sort(&mut arr);
		assert_eq!(arr, [1,2,3,4,5,6,7]);
	}

	#[test]
	fn test_random_array() {
		use rand::random;
		let mut s1 = [0; 100];

		for i in 0..s1.len() {
			s1[i] = random::<i32>();
		}

		let mut s2 = s1.clone();

		selection_sort(&mut s1);
		s2.sort();
		assert_eq!(s1, s2);
	}

	#[test]
	fn test_empty_array() {
		let mut s1 = [];
		selection_sort(&mut s1);
		assert_eq!(s1, []);
	}
}
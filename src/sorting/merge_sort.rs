fn merge(arr: &mut [i32], p: usize, q: usize, r: usize) {
	let left = arr[p..=q].to_vec();
	let right = arr[q+1..=r].to_vec();

	let mut i = 0;
	let mut j = 0;
	let mut k = 0;

	while i < left.len() && j < right.len() {
		if left[i] < right[j] {
			arr[k] = left[i];
			i += 1;
		} else {
			arr[k] = right[j];
			j += 1;
		}
		k += 1;
	}

	while i < left.len() {
		arr[k] = left[i];
		i += 1;
		k += 1;
	}

	while j < right.len() {
		arr[k] = right[j];
		j += 1;
		k += 1;
	}
}

pub fn merge_sort(arr: &mut [i32]) {
	let length = arr.len();

	if length > 1 {
		let mid = length / 2;
		merge_sort(&mut arr[..mid]);
		merge_sort(&mut arr[mid..]);
		merge(arr, 0, mid-1, length-1); // mid - 1 instead of mid
	}
}

#[cfg(not_test)]
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_merge() {
		let mut arr = [1, 3, 5, 7, 2, 4, 6, 8];
		merge(&mut arr, 0, 3, 7);
		assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8]);
	}

	#[test]
	fn test_merge_sort() {
		let mut arr = [1, 3, 5, 7, 2, 4, 6, 8];
		merge_sort(&mut arr);
		assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8]);
	}
}

//#[cfg(not_test)]
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_fixed_array() {
		let mut arr = [2, 1, 3, 6, 7, 4, 5];
		merge_sort(&mut arr);
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

		merge_sort(&mut s1);
		s2.sort();
		assert_eq!(s1, s2);
	}

	#[test]
	fn test_empty_array() {
		let mut s1 = [];
		merge_sort(&mut s1);
		assert_eq!(s1, []);
	}
}
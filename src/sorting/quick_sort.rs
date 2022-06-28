fn partition(arr: &mut [i32], p: usize, r: usize) -> isize {
	let key = r;
	let mut i: isize = p as isize - 1;

	for j in p..r {
		if arr[j] < arr[key] {
			i += 1;
			arr.swap(i as usize, j);
		}
	}
	arr.swap((i+1) as usize, key);
	i + 1
}

fn _quick_sort(arr: &mut [i32], p: isize, r: isize) {
	if p < r {
		let q = partition(arr, p as usize, r as usize);
		_quick_sort(arr, p, q-1); // q可能为0, q-1会溢出
		_quick_sort(arr, q+1, r);
	}
}

pub fn quick_sort(arr: &mut [i32]) {
	let length = arr.len();

	if length <= 1 {
		return;
	}

	_quick_sort(arr, 0, (length-1) as isize);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_fixed_array() {
		let mut arr = [2, 1, 3, 6, 7, 4, 5];
		quick_sort(&mut arr);
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

		quick_sort(&mut s1);
		s2.sort();
		assert_eq!(s1, s2);
	}

	#[test]
	fn test_empty_array() {
		let mut s1 = [];
		quick_sort(&mut s1);
		assert_eq!(s1, []);
	}
}
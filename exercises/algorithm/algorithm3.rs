/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


fn sort<T: Ord>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }
    
    let pivot_index = partition(array);
    let (left, right) = array.split_at_mut(pivot_index);
    sort(left);
    sort(&mut right[1..]); // 跳过基准元素
}
fn partition<T: Ord>(array: &mut [T]) -> usize {
    let last = array.len() - 1;
    let mid = array.len() / 2;
    array.swap(mid, last);
    
    let mut i = 0;
    for j in 0..last {
        if array[j] <= array[last] {
            array.swap(i, j);
            i += 1;
        }
    }
    
    array.swap(i, last);
    i
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
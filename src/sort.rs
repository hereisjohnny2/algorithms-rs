pub fn bubble_sort(arr: &mut [i32]) -> &mut [i32] {
    for i in 0..arr.len() {
        for j in 0..(arr.len() - 1 - i) {
            if arr[j] > arr[j + 1] {
                let tmp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = tmp;
            }
        }
    }

    arr
}


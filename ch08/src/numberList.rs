/*
 * 정수 리스트가 주어졌을 때, 벡터를 이용하여 이 리스트의 평균값(mean, average),
 * 중간값(median, 정렬했을 때 가장 가운데 위치한 값),
 * 그리고 최빈값(mode, 가장 많이 발생한 값; 해쉬맵이 여기서 도움이 될 것입니다)를 반환해보세요.
*/
use std::collections::HashMap;

fn main() {
	// let list: Vec<i32> = Vec::new();
	let mut list = vec![9,1,7,10,1,4,2,3,6,10,2,8,3,6,8,3,5,3,8,9,2];
	println!("Default: \n{:?}", list);
	
	list.sort();
	println!("Sorted: \n{:?}", list);

	let len = list.len() as f32;
	let sum = list.iter().sum::<i32>() as f32;
	let avg = sum / len;
	println!("\nAverage: {}, Sum: {}", avg, sum);

	let center_index = (len / 2.0).ceil() as usize;
	println!("Median: {}, Center index: {}", &list[center_index], center_index);

	let mut count = HashMap::new();
	let mut max_key:i32 = 0;
	let mut max_value:i32 = 0;

	for v in &list {
    let count = count.entry(v).or_insert(0);
    *count += 1;
		if max_value < *count {
			max_value = *count;
			max_key = *v;
		}
	}
	println!("Max Key: {}, Max Count: {}", max_key, max_value);
	// println!("Mode: {:?}", count);
}

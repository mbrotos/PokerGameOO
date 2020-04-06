use std::collections::HashSet;

fn get_rank(cardnum: u32) -> u32 {
	(cardnum-1)%13 + 1
}

fn is_sequence(arr: &[u32]) -> bool{
    let mut elem: u32 = arr[0];
	for i in 1..arr.len() {
        if elem+1 != arr[i] {return false;}
        elem = arr[i];
	}
	return true;
}

fn same_suit(arr: &[String]) -> bool {
	let set: HashSet<String> = arr.iter().cloned().collect();
	set.len() == 1
}

fn group_by_rank(arr: [u32; 5]) -> Vec<Vec<u32>> {
	let mut visited: [bool; 5] = [false; 5];
	let mut res: Vec<Vec<u32>> = vec![];

	for i in 0..5 {

		if visited[i] == false {

			let mut v: Vec<u32> = vec![];
			v.push(arr[i]);
			let rank: u32 = get_rank(arr[i]);

			for j in (i+1)..5 {
				if get_rank(arr[j]) == rank {
					visited[j] = true;
					v.push(arr[j]);
				}
			}

			res.push(v.clone());
		}
	}
    res.sort_by( |a, b| b.len().cmp(&a.len()));
	res
}

fn main(){
    let arr: [u32; 5] = [1,2,14,17, 43];
    let res = group_by_rank(arr);
    println!("{:?}", res);
}
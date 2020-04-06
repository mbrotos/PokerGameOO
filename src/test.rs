use std::collections::HashSet;

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

fn main(){
    let arr: [String; 3] = ["S".to_string(),"C".to_string(),"C".to_string()];
    let res: bool = same_suit(&arr);
    println!("{}", res);
}
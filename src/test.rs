use std::collections::HashSet;

struct PokerHand{
	data: [u32; 5],
	ranks: [u32; 5],
	suits: [String; 5],
}

impl PokerHand {
	pub fn new(cards: [u32; 5]) -> PokerHand{
		PokerHand {
			data: cards,
			ranks: get_ranks(cards),
			suits: get_suits(cards),
		}
	}
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

fn get_true_rank(cardnum: u32) -> u32{ //ace = 1 -> range [1-13]
	(cardnum-1)%13 + 1
}

fn get_true_ranks(arr: [u32; 5]) -> [u32; 5]{
	let mut res: [u32; 5] = [0; 5];
	for i in 0..5 {
		res[i] = get_true_rank(arr[i]);
	}
	res
}

fn get_rank(cardnum: u32) -> u32 {//ace = 14 -> range [2-14]
	let rank = (cardnum-1)%13 + 1;
	if rank == 1 { return 14;}
	else { return rank; }
}

fn get_ranks(arr: [u32; 5]) -> [u32; 5]{
	let mut res: [u32; 5] = [0; 5];
	for i in 0..5 {
		res[i] = get_rank(arr[i]);
	}
	res
}

fn get_suit(cardnum: u32) -> String {
	let div = (cardnum - 1)/13;
	let suit = if div == 0 {"C"}
			   else if div == 1 {"D"}
			   else if div == 2 {"H"}
			   else {"S"};
	suit.to_string()
}

fn get_suits(arr: [u32; 5]) -> [String; 5]{
	let mut res: [String; 5] = Default::default();
	for i in 0..5 {
		res[i] = get_suit(arr[i]);
	}
	res
}

fn sort_by_true_rank(arr: &mut [u32;5]) {
	arr.sort();
	arr.sort_by( |a, b| get_true_rank(*a).cmp(&get_true_rank(*b)));
}

fn rank_suit(hand: &PokerHand) -> [String; 5]{ //WORKS
	let mut res: [String; 5] = Default::default();
	let mut copy: [u32; 5] = hand.data.clone();
	sort_by_true_rank(&mut copy);
    let base: [u32; 5] = get_true_ranks(copy);
	let suits: [String; 5] = get_suits(copy);
	for i in 0..5 {
		let mut rank: String = base[i].to_string();
		let suit = &suits[i];
		rank.push_str(suit);
		res[i] = rank;
	}
	res
}

fn main(){
    // let arr: [u32; 5] = [1,2,14,17, 43];
    // let res = group_by_rank(arr);
    let hand: PokerHand = PokerHand::new([1,2,14,17, 43]);
    let res = rank_suit(&hand);
    println!("{:?}", res);
}
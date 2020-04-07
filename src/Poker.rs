use std::fmt;
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

pub fn deal(cards: [u32; 10]) -> [String; 5]{
	let (mut hand1, mut hand2) = hand_cards(cards);

	sort_by_rank(&mut hand1);
	sort_by_rank(&mut hand2);

	let (type1, type2): (u32, u32) = (get_type(&hand1), get_type(&hand2));
	println!("hand1: {}\thand2:{}", hand1, hand2);
	println!("type1: {}\t\ttype2: {}", type1, type2);

	let win: [String; 5] = if type1 > type2 { winner(&hand1) }
    else if type1 < type2 { winner(&hand2) }
	else { tie(&hand1, &hand2, type1) };
	println!("-----------------------------------------------");
	win
}

impl fmt::Display for PokerHand { //only for debugging
	fn fmt( &self, f: &mut fmt::Formatter) -> fmt::Result{
		
		let c:[String; 5] = rank_suit(self);
		write!(f, "[{}, {}, {}, {}, {}]", c[0], c[1], c[2], c[3], c[4])
	
	}
}

fn hand_cards(arr: [u32;10]) -> (PokerHand, PokerHand) {
	let mut arr1: [u32; 5] = [0; 5];
	let mut arr2: [u32; 5] = [0; 5];
	for i in 0..5 {
		arr1[i] = arr[2*i];
		arr2[i] = arr[2*i+1];
	}
	let hand1 = PokerHand::new(arr1);
	let hand2 = PokerHand::new(arr2);
	(hand1, hand2)
}

fn winner(hand: &PokerHand) -> [String; 5]{
	let s: [String; 5] = rank_suit(hand);
	println!("\nWINNER: {:?}", s);
	s
}

fn get_type(hand: &PokerHand) -> u32 {
	//hand should be sorted by rank
	if is_royalFlush(hand) {9}
    else if is_straightFlush(hand) {8}
    else if is_fourOfKind(hand) {7}
    else if is_fullHouse(hand) {6}
    else if is_flush(hand) {5}
    else if is_straight(hand) {4}
    else if is_threeOfKind(hand) {3}
    else if is_twoPair(hand) {2}
    else if is_pair(hand) {1}
    else {0}
}

// ---------------------------------------------------------

fn is_royalFlush(hand: &PokerHand) -> bool {
	let base: [u32; 5] = hand.ranks;
    if base == [10, 11, 12, 13, 14]  && same_suit(&hand.suits) {return true;}
    else {return false;} 
}

fn is_straightFlush(hand: &PokerHand) -> bool {
    if is_flush(hand) && is_straight(hand) {return true;}
    else {return false;}
}

fn is_fourOfKind(hand: &PokerHand) -> bool {
	let base: [u32; 5] = hand.ranks;
	if same_rank(&base[..4]) || same_rank(&base[1..]) {return true;}
    else {return false;}
}

fn is_fullHouse(hand: &PokerHand) -> bool {
	let base: [u32; 5] = hand.ranks;
	if same_rank(&base[..3]) && same_rank(&base[3..]) {return true;}
	else if same_rank(&base[..2]) && same_rank(&base[2..]) {return true;}
    else {return false;}
}

fn is_flush(hand: &PokerHand) -> bool {
    same_suit(&hand.suits)
}

fn is_straight(hand: &PokerHand) -> bool {
	let base: [u32; 5] = hand.ranks;
	is_sequence(&base[..]) || base == [2, 3, 4, 5, 14]
}

fn is_threeOfKind(hand: &PokerHand) -> bool {
    let base: [u32; 5] = hand.ranks;
    let uniqHand = dedup(hand);
    if uniqHand.len() == 3 
    && uniqHand.iter().any(|x| base.iter().filter(|y| **y==*x).count() == 3)
    {return true;}
    else {return false;}
}

fn is_twoPair(hand: &PokerHand) -> bool {
    if dedup(hand).len() == 3 {return true;}
    else {return false;}
}

fn is_pair(hand: &PokerHand) -> bool {
    if dedup(hand).len() == 4 {return true;}
    else {return false;}
}
// ---------------------------------------------------------
//change
fn tie(hand1: &PokerHand, hand2: &PokerHand, t: u32) -> [String; 5]{	
	match t {
		9 => tie9(hand1, hand2),
		8 => tie8(hand1, hand2),
		6|7 => tie67(hand1, hand2),
		4 => tie4(hand1, hand2),
		_=> tie0(hand1, hand2),
	}
}

//tie for Royal Flush
fn tie9(hand1: &PokerHand, hand2: &PokerHand) -> [String; 5]{
	if hand1.data[4]>hand2.data[4] {return winner(&hand1);}
	else {return winner(&hand2);}
}

//Straight Flush
fn tie8(hand1: &PokerHand, hand2: &PokerHand) -> [String; 5]{
	let card1: u32 = get_highest_card(hand1);
	let card2: u32 = get_highest_card(hand2);
	let rank1: u32 = get_true_rank(card1);
	let rank2: u32 = get_true_rank(card2);
	let res: &PokerHand = 
		if rank1 == rank2 {  // if all ranks are same - check suits
			if card1>card2 {hand1}
			else {hand2}
		}
		else if rank1 > rank2 {hand1}
		else {hand2};
	winner(res)
}

//Four of a kind & Full House
fn tie67(hand1: &PokerHand, hand2: &PokerHand) -> [String; 5]{
	let card1: u32 = get_highest_card(hand1);
	let card2: u32 = get_highest_card(hand2);
	if get_rank(card1) > get_rank(card2) {return winner(hand1);}
	else {return winner(hand2);}
}

//Straight
fn tie4(hand1: &PokerHand, hand2: &PokerHand) -> [String; 5]{
	let mut copy1: Vec<u32> = hand1.ranks.clone().to_vec();
	let mut copy2: Vec<u32> = hand2.ranks.clone().to_vec();
	if copy1[4] == 14 && copy1[3]==5 { copy1.remove(4); copy1.insert(0,1);} //moves ace to the begining if [2,3,4,5,14]
	if copy2[4] == 14 && copy2[3]==5 { copy2.remove(4); copy2.insert(0,1);}
	for i in (0..5).rev() {
		if copy1[i] > copy2[i] {return winner(hand1);}
		else if copy1[i] < copy2[i] {return winner(hand2);}
		else {continue;}
	}
	tie_by_suit(hand1, hand2)
}

//Flush,HighCard
fn tie0(hand1: &PokerHand, hand2: &PokerHand) -> [String; 5]{
	for i in (0..5).rev() {
		if hand1.ranks[i] > hand2.ranks[i] {return winner(hand1);}
		else if hand1.ranks[i] < hand2.ranks[i] {return winner(hand2);}
		else {continue;}
	}
	tie_by_suit(hand1, hand2)
}

fn tie_by_suit(hand1: &PokerHand, hand2: &PokerHand) -> [String; 5]{
	let card1: u32 = get_highest_card(hand1);
	let card2: u32 = get_highest_card(hand2);
 	if card1>card2 {return winner(hand1);}
 	else {return winner(hand2);}
}

fn get_highest_card(hand: &PokerHand) -> u32{
	match get_type(hand) {
		8 => {
			if hand.ranks[4] == 14 {return hand.data[3];}
			else {return hand.data[4];}
		}
		6|7 => {
			let grouped: Vec<Vec<u32>> = group_by_rank(hand.data.clone());
			//println!("grouped:{:?}", grouped);
			let res = *grouped[0].iter().max().unwrap();
			//println!("returned: {}", res);
			res
		}
		5 => {
			hand.data[4]
		}
		4 => {
			if hand.ranks[4] == 14 && hand.ranks[3] == 5 { return hand.data[3];} 
			else {return hand.data[4];}
		}
		_ => {hand.data[4]}
	}
}
// ---------------------------------------------------------
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


fn sort_by_rank(hand: &mut PokerHand) {
	hand.data.sort();
	hand.data.sort_by( |a, b| get_rank(*a).cmp(&get_rank(*b)));
	hand.ranks = get_ranks(hand.data);
	hand.suits = get_suits(hand.data);
}

fn sort_by_true_rank(arr: &mut [u32;5]) {
	arr.sort();
	arr.sort_by( |a, b| get_true_rank(*a).cmp(&get_true_rank(*b)));
}

fn same_suit(arr: &[String]) -> bool {//WORKS
	let set: HashSet<String> = arr.iter().cloned().collect();
	set.len() == 1
}

fn same_rank(arr: &[u32]) -> bool{//WORKS
	let set: HashSet<u32> = arr.iter().cloned().collect();
	set.len() == 1
}

fn is_sequence(arr: &[u32]) -> bool{//WORKS
    let mut elem: u32 = arr[0];
	for i in 1..arr.len() {
        if elem+1 != arr[i] {return false;}
        elem = arr[i];
	}
	return true;
}

fn dedup(hand: &PokerHand) -> Vec<u32> {
	let mut result = vec![];

    let mut seen = HashSet::new();
    for elem in hand.ranks.iter() {
        if !seen.contains(elem) {
            result.push(*elem);
            seen.insert(elem);
        }
    }
    result
}

fn group_by_rank(arr: [u32; 5]) -> Vec<Vec<u32>> { //WORKS
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
// ---------------------------------------------------------
use std::fmt;
use std::collections::HashSet;

struct PokerHand{
	data: [u32; 5],
	ranks: [u32; 5],
	suits: [String; 5],
}

pub fn deal(cards: [u32; 10]) -> [String; 5]{
	let (mut hand1, mut hand2) = hand_cards(cards);
	let (type1, type2): (u32, u32) = (get_type(&hand1), get_type(&hand2));
	
	println!("hand1: {:?}\thand2: {:?}", hand1.data, hand2.data);
	println!("hand1: {}\thand2: {}", hand1, hand2);

	sort_by_rank(&mut hand1);
	sort_by_rank(&mut hand2);

	println!("\nSORTED:\nhand1: {} type: {}\nhand2: {} type: {}", hand1, type1, hand2, type2);

	let win: [String; 5] = if type1 > type2 { winner(&hand1) }
    else if type1 < type2 { winner(&hand2) }
    else { tie(&hand1, &hand2, type1) };
	win
}

impl fmt::Display for PokerHand { //only for debugging
	fn fmt( &self, f: &mut fmt::Formatter) -> fmt::Result{
		
		let mut c:[String; 5] = rank_suit(self);
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
	let hand1 = PokerHand { data: arr1, ranks: get_ranks(arr1), suits: get_suits(arr1), };
	let hand2 = PokerHand { data: arr2, ranks: get_ranks(arr2), suits: get_suits(arr2), };
	(hand1, hand2)
}

fn winner(hand: &PokerHand) -> [String; 5]{
	//sort_by_rank(hand);
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
    if base == [1, 10, 11, 12, 13] && same_suit(&hand.suits[..]) {return true;}
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
	is_sequence(&base[..]) || base == [1, 10, 11, 12, 13]
}

fn is_threeOfKind(hand: &PokerHand) -> bool {
    let base: [u32; 5] = hand.ranks;
    same_rank(&base[..3]) || same_rank(&base[1..4]) || same_rank(&base[2..])
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
	winner(hand1)
}

fn tie_by_suit(hand1: &PokerHand, hand2: &PokerHand, t: u32) -> [String; 5]{
	let card1: u32 = get_highest_card(hand1, t);
	let card2: u32 = get_highest_card(hand2, t);
	if card1>card2 {return winner(hand1);}
	else {return winner(hand2);}
}
// ---------------------------------------------------------
fn get_rank(cardnum: u32) -> u32 {
	(cardnum-1)%13 + 1
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

fn rank_suit(hand: &PokerHand) -> [String; 5]{
	let mut res: [String; 5] = Default::default();
	for i in 0..5 {
		let mut rank: String = hand.ranks[i].to_string();
		let suit = &hand.suits[i];
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

//change
fn get_highest_card(hand: &PokerHand, t: u32) -> u32{
	hand.data[0]
}
// ---------------------------------------------------------
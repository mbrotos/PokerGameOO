use std::collections::HashSet;

fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut result = vec![];

    let mut seen = HashSet::new();
    for elem in vs {
        if !seen.contains(elem) {
            result.push(*elem);
            seen.insert(elem);
        }
    }
    result
}

fn baseHand(hand: [i32;5]) -> Vec<i32> {
    hand.iter().map(|x| x % 13).collect::<Vec<i32>>()
}

fn is_flush(hand: [i32;5]) -> bool {
    let mut iterHand = hand.iter();
    if iterHand.all(|x| x>=&1 && x <=&13)
    || iterHand.all(|x| x>=&14 && x <=&26)
    || iterHand.all(|x| x>=&27 && x <=&39)
    || iterHand.all(|x| x>=&40 && x <=&52) {return true;}
    else {return false;}
}

fn is_straight(hand: [i32;5]) -> bool {
    let mut bHand = baseHand(hand);
    bHand.sort();
    //still need to do this

}

fn is_threeOfKind(hand: [i32;5]) -> bool {
    let mut bHand = baseHand(hand);
    bHand.sort();
    let uniqHand = dedup(&bHand.clone());
    if uniqHand.len() == 3 
    && uniqHand.iter().any(|x| bHand.iter().filter(|y| **y==*x).count() == 3)
    {return true;}
    else {return false;}
}

fn is_twoPair(hand: [i32;5]) -> bool {
    let mut bHand = baseHand(hand);
    bHand.sort();
    if dedup(&bHand) == 3 {return true;}
    else {return false;}
}

fn is_pair(hand: [i32;5]) -> bool {
    let mut bHand = baseHand(hand);
    bHand.sort();
    if dedup(&bHand) == 4 {return true;}
    else {return false;}
}

fn is_royalFlush(hand: [i32;5]) -> bool {
    let sortedBase = baseHand(hand);
    sortedBase.sort();
    if is_flush(hand) 
    && is_straight(hand)
    && sortedBase == vec![0,1,10,11,12] {return true;}
    else {return false;}
}

fn is_straighFlush(hand: [i32;5]) -> bool {
    if is_flush(hand) && is_straight(hand) {return true;}
    else {return false;}
}

fn is_fourOfKind(hand: [i32;5]) -> bool {
    let mut bHand = baseHand(hand);
    bHand.sort();
    let uniqHand = dedup(&bHand.clone());
    if uniqHand.len() == 2 
    && uniqHand.iter().any(|x| bHand.iter().filter(|y| **y==*x).count() == 4)
    {return true;}
    else {return false;}
}

fn is_fullHouse(hand: [i32;5]) -> bool {
    let mut bHand = baseHand(hand);
    bHand.sort();
    let uniqHand = dedup(&bHand.clone());
    if uniqHand.len() == 2 
    && uniqHand.iter().any(|x| bHand.iter().filter(|y| **y==*x).count() == 3)
    {return true;}
    else {return false;}
}

fn getType(hand: [i32;5]) -> i32 {
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

fn deal(hands: [i32;10]) -> [&str;5]
{
    let hand1 = &hands[0..5];
    let hand2 = &hands[5..10];
    let hand1Type = getType(hand1);
    let hand2Type = getType(hand2);
    
    if (hand1Type > hand2Type)  { return output(hand1Type); }
    else if (hand1Type < hand2Type) { return output(hand2Type; )}
    else { return output(tieBreak(hand1, hand2, hand1Type)); }
}

//To be removed
fn main() {
    let perm:[u32;10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let winner:[&str;5] = deal(perm);
}
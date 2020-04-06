mod Poker;
use Poker::deal;

fn main() {
	deal([32, 40, 11, 2, 33, 17, 1, 9, 6, 50]);
	deal([36, 25, 39, 26, 27, 23, 38, 24, 37, 14]); //royal flushes
	deal([1, 14, 2, 15, 3, 16, 4, 17, 5, 18]);
	deal([5, 40, 7, 42, 6, 43, 3, 41, 4, 44]); //straight flushes
	deal([ 13,11, 5, 24, 18, 37, 31, 50, 44, 3]); //four of a kind
	deal([ 22, 28, 45, 15, 35, 13, 48, 2, 6, 26 ]); 
}

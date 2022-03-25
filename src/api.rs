const WORD: &str = "depot";

pub fn try_guess(word: &str) -> Vec<u8> {
	word.chars()
		.enumerate()
		.map(|(i, x)| match x == WORD.chars().nth(i).unwrap() {
			true => 2,
			false => WORD.contains(x) as u8
		})
		.collect()
}
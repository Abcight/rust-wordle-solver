pub mod api;

fn solve(words: &mut std::vec::Vec<&str>) {
	if let Some(word) = words.get(0) {
		let word = word.to_owned();
		let res = api::try_guess(word);
		
		println!("{} {:?}", word, res);
		
		if res.iter().sum::<u8>() == 10 {
			return;
		}

		res.iter().enumerate().for_each(|(i, score)| {
			let ch = word.chars().nth(i).unwrap();
			words.retain(|w| match score {
				0 => !w.contains(ch),
				1 => w.chars().nth(i).unwrap() != ch && word.contains(ch),
				_ => w.chars().nth(i).unwrap() == ch
			});
		});

		solve(words);
	}
}

fn main() {
	if let Ok(content) = std::fs::read_to_string("./words.txt") {
		solve(&mut content.lines().collect());
	}
}
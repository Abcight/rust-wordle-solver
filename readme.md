# Naive Wordle Solver
A simple [Wordle](https://www.nytimes.com/games/wordle/index.html) game solving algorithm written in pure Rust with zero dependencies.

## The algorithm
At the beggining, we load all the valid wordle inputs from ``words.txt`` and store them in a vector. Each attempt at guessing a word, the API layer will return a vector of integers, each corresponding to the color output we would have gotten had we played wordle -- 0 for dark gray, 1 for yellow and 2 for green. We then match these outputs against the individual characters from the input; We then remove words from the vector we populated in the beggining based on the conclusions we drawn from matching the input word's individual characters against the output vector; If character at index n corresponds to a score of...
- 0, we remove every word that contains the character.
- 1, we remove every word that contains that character at index n and every word that doesn't contain the character at any index.
- 2, we remove every word that doesn't contain that character at index n.

We repeat these steps until the API layer responds with a vector that has the sum of its values equal to 10 (5 times the highest possible score of 2 -- all green).

## The API
### or thereby the lack of it...
Wordle doesn't provide an official API - the original game is served statically and played right in your browser. For this reason, I have created a thin layer of abstraction that resides in ``api.rs``, which simply checks the input against a constant defined in that file. If you want to use a 3rd party wordle API, you'll have to implement a function fetching and processing the data yourself.
fn main() {
    // 2. Options
        // 2.1 : Filtrage par .match()
        let sentence1 : &str = "Bonjour Limoges";
        let sentence2 : &str = "";
        print_first_word1(sentence1);
        print_first_word1(sentence2);

        // 2.2 : Exigence par .expect()
        print_first_word2(sentence1);
        print_first_word2(sentence2);
}

//2. Options
fn print_first_word1(sentence : &str) {
    let first_word: &str = sentence.split_whitespace().next().unwrap_or_else(|| "Chaine vide");
    println!("Premier mot : {}", first_word);
}

fn print_first_word2(sentence : &str) {
    let first_word: &str = sentence.split_whitespace().next().expect("La chaine doit être non vide");
    println!("Premier mot {}", first_word)
}
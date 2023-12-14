mod domain;
mod infrastructure;
use infrastructure::word_set_file_repo::WordSetFileRepo;

fn main() {
    let problem_path = "Problem";
    let data_path = "Database";

    let problem = WordSetFileRepo::new(problem_path);
    let data = WordSetFileRepo::new(data_path);

    let problem_set = problem.get_word_set();
    let data_set = data.get_word_set();

    println!("問題の単語数:{}", problem_set.get_count());
    println!("データの数:{}", data_set.get_count());

    let inter = problem_set.return_match(&data_set);
    let cover = (inter.len() as f64) / (problem_set.get_count() as f64) * 100.0;
    println!("カバー率:{}", cover);
    println!("一致しなかった単語");
    for word in problem_set.return_nonmatch(&data_set) {
        println!("{}", word.contents);
    }

    let mut last = String::new();
    std::io::stdin().read_line(&mut last).ok();
}

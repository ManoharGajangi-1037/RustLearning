use std::sync::{Arc, Mutex};
use std::thread;
use std::{fs, vec};

fn count_word_occurences(contents: &str, word_to_count: &str) -> usize {
    contents
        .split_whitespace()
        .filter(|w| *w == word_to_count)
        .count()
}
fn main() {
    let files = vec!["data/file1.txt", "data/file2.txt"];
    let word_to_count = "Manohar";
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for file_path in files {
        let path = file_path.to_string();
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || match fs::read_to_string(&path) {
            Ok(contents) => {
                let local_count = count_word_occurences(&contents, &word_to_count);
                let mut total_count = counter_clone.lock().unwrap();
                *total_count += local_count;
            }
            Err(_) => println!("Errot Occured"),
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap();
    }
    let total_word_count=*counter.lock().unwrap();
    println!("Total Word Count :{}",total_word_count);
}

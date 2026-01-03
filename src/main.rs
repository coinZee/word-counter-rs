
fn count_word(s: &str) -> usize{
    let mut count: i8 = 0;
    let mut in_word = false;

    for&item in s.as_bytes() {
        if item == b' ' {
            in_word = false;
        } else if !in_word {
            in_word = true;
            count += 1;
        }
    }
    count.try_into().unwrap()
}

fn main(){
    let mut s = String::from("Hello    world!    ls  ");
    let mut word_count: usize = 0;
    for _ in [1..100]{
        word_count = count_word(&s);
    }
    println!("{word_count}");
    s.clear();
}

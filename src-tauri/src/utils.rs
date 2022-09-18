use rand::Rng;


pub fn generate_random_string(len: i8) -> String {
    let chars: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyz".chars().collect();
    let chars_len = chars.len();
    let mut chosen: String = String::new();
    for _ in 1..=len {
        let rand_num = rand::thread_rng().gen_range(0..chars_len);
        chosen.push(chars[rand_num]);
    }
    chosen
}

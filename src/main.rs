use rand::seq::SliceRandom;

fn main() {
    let deer = ["し","か","の","こ","た","ん"];
    let mut result = String::new();
    for _ in 1..15 {
        let str = deer.choose(&mut rand::thread_rng()).unwrap();
        result.push_str(&str);
    }
    println!("{:?}",result);
}

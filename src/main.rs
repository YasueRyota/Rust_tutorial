use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");          // 数を当ててごらん

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Please input your guess.");   // ほら、予想を入力してね

    loop {

        let mut guess = String::new();          // // mutはmutable、つまり可変という意味

        io::stdin()
            .read_line(&mut guess)              // // ここでもmut、可変で渡す
            .expect("Failed to read line");     // 行の読み込みに失敗しました

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");                 //数値を入力してください！
                continue;
            }
        };

        println!("You guessed: {}", guess);     // 次のように予想しました: {}

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),       //小さすぎ！
            Ordering::Greater => println!("Too big!"),      //大きすぎ！
            Ordering::Equal => {
                println!("You win!");                       //やったね！
                break;
            }
        }
    }
}

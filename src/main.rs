mod guess;
use guess::Guess;

use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // 1から100までの乱数
    // 下限値は含まれるが上限値は含まれないことに注意
    let secret_number = rand::rng().random_range(1..101);
    // こういう書き方も出来る
    // let secret_number = rand::rng().random_range(1..=100);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            // memo 参照もデフォルトで不変なのでmutを付ける必要がある
            .read_line(&mut guess)
            // expectを書かないと警告が出る
            // エラーの場合にクラッシュさせて、引数のメッセージを表示する
            .expect("Failed to read line");

        // trimで空白や改行文字を削除
        let guess: Guess = match guess.trim().parse() { 
            Ok(num) => {
                match Guess::new(num) {
                    Ok(guess) => guess,
                    Err(error) => {
                        println!("{:?}", error);
                        continue
                    }
                }
            },
            Err(error) => {
                println!("{:?}", error);
                continue
            },
        };

        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => { println!("Too small!") }
            Ordering::Greater => { println!("Too big!") }
            Ordering::Equal => { 
                println!("You win!");
                break;
            }
        }
    }
}

use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        // memo 参照もデフォルトで不変なのでmutを付ける必要がある
        .read_line(&mut guess)
        // expectを書かないと警告が出る
        // エラーの場合にクラッシュさせて、引数のメッセージを表示する
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

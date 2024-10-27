use rand::Rng;

fn main() {
    let op1 = rand::thread_rng().gen_range(0..100);
    let op2 = rand::thread_rng().gen_range(0..100);

    // println!("Hello, world!");
    println!("{} + {} = ??", op1, op2);
    println!("?? の値を入力してください:");
    let mut ans_input = String::new();

    // 標準入力から入力受け取る
    std::io::stdin().read_line(&mut ans_input).unwrap();

    // 改行コードを取り除いて数値へ変換
    let ans_input = ans_input.trim().parse::<i32>().unwrap();

    dbg!(ans_input);
    // 表示
    if dbg!(ans_input == op1 + op2) {
        println!("正解")
    } else {
        println!("不正解")
    }

    // 減産
    let op1 = rand::thread_rng().gen_range(0..100);
    let op2 = rand::thread_rng().gen_range(0..100);
    println!("{} - {} = ??", op1, op2);
    println!("?? の値を入力してください:");
    let mut ans_input = String::new();

    // 標準入力から入力受け取る
    std::io::stdin().read_line(&mut ans_input).unwrap();

    // 改行コードを取り除いて数値へ変換
    let ans_input = ans_input.trim().parse::<i32>().unwrap();

    dbg!(ans_input);

    // 表示
    if dbg!(ans_input == op1 - op2) {
        println!("正解")
    } else {
        println!("不正解")
    }
    // println!("i32 range: {} ~ {}", i32::MIN, i32::MAX);
    // println!("u32 range: {} ~ {}", u32::MIN, u32::MAX);
}

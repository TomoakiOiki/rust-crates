fn main() {}

// 参照を返す場合ライフタイムがあっていないとエラーになる
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
// 下記は所有権を渡すのでOK
// fn dangle() -> String {
//     let s = String::from("hello");
//     s
// }

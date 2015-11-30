extern crate chinese_num;

fn main(){
  for arg in std::env::args().skip(1) {
    match chinese_num::to_chinese_num(&arg) {
      Some(s) => println!("{}: {}", arg, s),
      None => println!("{}: 不是一个数", arg),
    }
  }
}

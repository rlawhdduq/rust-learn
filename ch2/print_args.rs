// 48. 명령줄 인수를 표시만 하는 프로그램
use std::env;

fn main(){
    let args = env::args();
    for arg in args {
        println!("{}", arg);
    }
}
// 46. 에러가 발생하는 프로그램 - 예외처리 - match 사용
use std::collections::HashMap;

fn main(){
    // HashMap을 생성해 초기화
    let mut map = HashMap::new();
    map.insert("A", 30);
    map.insert("B", 50);

    match map.get("D") {
        Some(v) => println!("D={}", v),
        None => println!("D는 존재하지 않습니다."),
    }
}
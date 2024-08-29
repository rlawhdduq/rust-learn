// 6. (부록) println! 매크로 사용 방법 정리
// println! 함수는 소유권 문제가 발생하지 않게 구현되어있다.
// 얼마든지 사용이 가능하다!

fn main(){
    let txt = "서로 사랑하면 살고 서로 싸우면 죽는다".to_string();
    println!("{}", txt);
    println!("{}", txt);
}

// 6. (부록) println! 매크로 사용 방법 정리
// println! 함수는 소유권 문제가 발생하지 않게 구현되어있다.

// println!을 모방해 echo함수를 만들어 사용하는 예이다. 단, 해당 함수는 소유권이 이동하기 때문에 에러가 발생한다.

fn main(){
    let txt = "서로 사랑하면 살고 서로 싸우면 죽는다".to_string();
    echo(txt);
    println!("{}", txt);
}

fn echo(s: String){
    println!("{}", s);
}
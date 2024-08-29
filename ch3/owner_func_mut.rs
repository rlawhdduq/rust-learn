// 6. 가변 참조자를 인수로 사용하기
// 함수 호출에 참조자를 사용할 때 함수 안에서 인수의 값을 변경해야 하는 경우도 있따.
// 이때는 참조자가 가변이라는 것을 명시하면 함수 안에서 인수의 값을 변경할 수 있다.

fn add_quote(msg: &mut String) {
    msg.insert(0, '"');
    msg.push('"');
}

fn main(){
    let mut msg = String::from("건강한 신체에 건강한 정신이 깃든다.");
    println!("{}", msg);
    add_quote(&mut msg);
    println!("{}", msg);
}
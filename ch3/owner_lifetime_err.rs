// 5. 참조자를 반환하는 함수
// 함수에서 참조자를 반환해야 하는 경우도 있다.
// 예를 들어 어떤 함수 안에서 데이터를 만들고, 값이 아니라 그 참조자를 반환해야 하는 경우.
// 근데 러스트에서는 값에 수명(Lifetime)이 있다

// 메시지를 생성한 뒤 그 참조자를 반환하는 함수
fn gen_message() -> &str {
    let msg = String::from("실수할 줄 아는 사람이 아름답다");
    return &msg;
}

fn main(){
    let m = gen_message();
    println!("{}", m);
}
// 21. 참조 예시
fn main(){
    // 변수 v를 10으로 설정
    let mut v = 10;
    println!("v={}", v);

    // 함수를 호출
    set_value(&mut v);
    println!("v={}", v);
}

fn set_value(arg: &mut u32){
    *arg = 100
}
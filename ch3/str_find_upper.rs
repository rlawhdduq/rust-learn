// 12. 클로저를 지정한 검색
// find함수에는 클로저를 지정할 수도 있다.
fn main(){
    // 변수 s에 문장을 대입
    let s = format!("{}{}", "There is more happiness in giving", "than there is in receving");

    // 클로저로 검색
    let res = s.find(|c:char| c.to_ascii_uppercase() == 'S');
    match res {
        Some(i) => println!("S={}B", i),
        None => println!("None")
    };
}
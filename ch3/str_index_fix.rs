// 11. 문자열에서 문자 한 개씩 가져오기
// 러스트의 인코딩은 UTF-8
// 영어나 숫자는 N바이트 째 문자 = N번째 문자
// 한글이나 일본어(3바이트)는 N바이트 째 문자 != N번째 문자
fn main(){
    let s = "안녕하세요";

    // 첫 1글자를 가져온다.
    let ch = s.chars().nth(0).unwrap();
    println!("{}", ch); // 안
    
    // 3번째 글자를 가져온다
    let ch2 = s.chars().nth(2).unwrap();
    println!("{}", ch2); // 하
}
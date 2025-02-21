// 12. 문자열 검색
// find함수를 이용하여 &str, char 타입 문자열을 검색해본다. 
fn main(){
    let s = "제주도의 특산품 중 귤은 겨울에 많이 먹을 수 있다.";

    // '귤'을 검색
    match s.find('귤') {
        Some(i) => println!("귤 = {}B", i),
        None => println!("'귤' 이라는 단어는 없습니다.")
    }

    // "바나나"를 검색
    match s.find("바나나") {
        Some(i) => println!("바나나 = {}B", i),
        None => println!("'바나나'라는 단어는 없습니다.")
    }
}
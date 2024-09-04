// 12. 부분 문자열 얻기
// chars와 enumerate를 이용해 부분 문자열을 얻는 방법
fn main(){
    let pr = "🥺🐇🧸🥇🤣"; 

    // 앞의 2글자(6바이트)를 얻기
    let mut sub1 = String::new();
    for (i, c) in pr.chars().enumerate() {
        if i < 2 { sub1.push(c); continue; }
        break;
    }
    println!("앞 2글자: {}", sub1);

    // 🥇🤣 부분 얻기
    let mut sub2 = String::new();
    for (i, c) in pr.chars().enumerate() {
        if 3 <= i && i <= 4 { sub2.push(c); }
    }
    println!("4~5번째 문자 : {}", sub2);
}
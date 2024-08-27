// 47. 러스트로 명령줄 인수 다루기
fn main(){
    // 명령줄 인수 취득
    let args = std::env::args();
    let mut total = 0.0;

    // 명령줄 인수를 순서대로 연산
    for (i, v) in args.enumerate() {
        // 0번째는 명령어(프로그램) 자신이므로 무시
        println!("{}", v); // -> 뭐라고 찍힐 지 궁금해서 넣어봄
        if i == 0 { continue; }

        // 책 내용
        // let mut num: f64 = match v.parse() {
        //     Ok(v) => v,
        //     Err(_) => 0.0,
        // }
        // total += num;

        // 하단은 내가 줄여본거
        // 명령줄 인수를 숫자로 변환
        match v.parse::<f64>() {
            Ok(v) => total += v,
            Err(_) => println!("문자열 발견!"),
        };
        
    }
    
    // 결과 표시
    println!("total => {}", total);
}
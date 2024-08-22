// 39. 문자열 파싱
// Rust의 try catch문이 궁금해져서 찾아보고 적용시켜보았다. -> 실습 바로뒤에 Result타입에 대한 설명과 예문이 있었다...
// Rust는 try catch문이 없다(!!)
// 대신 Result 타입과 ? 연산자를 사용하여 에러를 처리한다고 한다.
// fn main(){
//     // 문자열 지정
//     let s = "1stra";

//     let result = (|| -> Result<f64, &str> {
//         // f64타입으로 변환
//         let num: f64 = s.parse().map_err(|_| "변환 실패")?;
//         Ok(num)
//     })();

//     match result {
//         Ok(num) => println!("{}", num),
//         Err(e) => eprintln!("{}", e)
//     }
// }

// 책에서 소개하는 에러 핸들링 방법
fn main(){
    let s = "3.1415";
    let num = s.parse::<f64>();
    match num {
        Ok(result) => println!("{:.2}", result),
        Err(e) => println!("에러가 발생했습니다. 이유 : '{:?}'", e)
    }
}
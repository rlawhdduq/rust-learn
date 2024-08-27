// 50. 지정한 텍스트 파일의 내용을 읽어와 표시
use std::env; // 명령줄 인수 취득용
use std::fs; // 파일 읽기용

fn main(){
    // 인수를 벡터로 취득
    let args: Vec<String> = env::args().collect();

    // 인수를 지정했는지 확인
    if args.len() < 2 {
        println!("읽어 올 파일을 지정 해 주세요.");
        return;
    }

    // 두 번째 요소
    let filename = &args[1];
    
    // 파일을 읽어와 출력
    match fs::read_to_string(filename) {
        Ok(v)  => println!("{}", v),
        Err(e) => println!("{}", e.to_string()),
    }
}
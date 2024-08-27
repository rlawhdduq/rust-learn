// 51. 러스트로 텍스트 파일 안의 숫자를 더하는 도구 만들기
use std::{env, fs};

fn calc_num(total: &mut f64, num: f64) {
    *total += num;
}

fn main(){
    // 명령줄 인수 취득
    let args = env::args();
    let mut total: f64 = 0.0;
    
    // 모든 인수를 처리
    for (i, name) in args.enumerate() {
        if i == 0 { continue; }

        // 텍스트 파일을 읽어들임
        let text = fs::read_to_string(name).unwrap();
        // 한 줄씩 분리
        let lines = text.split("\r\n");

        for line in lines {
            // 숫자 값으로 변경
            // let n: f64 = match line.parse() {
            //     Ok(v)  => v,
            //     Err(_) => 0.0,
            // };
            // total += n;

            // 교재내용 살짝 바꿔봄
            // match line.parse::<f64>() {
            //     Ok(v)  => total += v,
            //     Err(_) => total += 0.0,
            // }

            // 교재내용 살짝 바꿔봄2
            match line.parse::<f64>() {
                Ok(v)  => calc_num(&mut total, v),
                Err(_) => calc_num(&mut total, 0.0),
            }
        }

    }

    println!("{}", total);
}
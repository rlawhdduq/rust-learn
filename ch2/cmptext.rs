// 52. 러스트로 2개의 텍스트 파일 비교하기
// 파일 조작용 라이브러리 이용 선언
use std::fs;

fn main(){
    // 파일 이름 지정
    let afile = "./fizzbuzz_python.txt";
    let bfile = "./fizzbuzz_rust.txt";

    // 파일 내용을 문자열로 읽어들임
    let astr = fs::read_to_string(afile).unwrap();
    let bstr = fs::read_to_string(bfile).unwrap();
    // 읽어들인 내용을 가공하지 않고 그냥 출력 -> 추가실습
    // println!("RAW DATA : {:?}", astr);
    // println!("RAW DATA : {:?}", bstr);
    // println!("UNWRAP : {:?}", astr.unwrap());
    // println!("UNWRAP : {:?}", bstr.unwrap());

    // 불필요한 공백 삭제
    let astr = astr.trim();
    let bstr = bstr.trim();

    // 비교
    if astr == bstr {
        println!("ok");
    } else {
        println!("ng");
    }
}
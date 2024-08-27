// 49. 명령줄 인수를 벡터 타입으로 만들기
fn main(){
    // 명령줄 인수를 Vec<String> 타입으로 취득
    let args: Vec<String> = std::env::args().collect();

    // 책 내용
    println!("{:?}", args);

    // 하단은 0번 제외하고부터 호출할 순 없나 해서 찾아보고 적용시켜 본 내용
    println!("{:?}", &args[1..]);
}
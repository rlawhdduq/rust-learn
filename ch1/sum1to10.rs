// 22. 포문 다루기
fn main(){
    let mut total = 0;
    // for i in 1..11 { // 이렇게 할 경우 이 값이 10까지인지 11까지인지 헷갈린다
    for i in 1..=10 { // 그래서 1..=10 이런식으로 표현할 수 있다
        total += i;
    }
    println!("{}", total);
}
// 56. 재귀를 이용해 피보나치 수열 구하기

// 피보나치 수를 구하는 함수
fn fib(n: i64) -> i64 {
    if n == 1 { return 0; } // 재귀 호출 종료 조건
    if n == 2 { return 1; }
    return fib(n-2) + fib(n-1); // 재귀 호출
}

fn main(){
    for i in 1..=20 {
        println!("{}", fib(i));
    }
}
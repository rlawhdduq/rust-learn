// 55. 재귀를 이용해 1부터 10까지의 수를 더하기

// 재귀적으로 호출 할 함수 sum
fn sum(n: i32) -> i32 {
    if n <= 0 { return 0; } // 재귀 종료 조건
    return sum(n -1) + n;
}

fn main(){
    println!("{}", sum(10));
}
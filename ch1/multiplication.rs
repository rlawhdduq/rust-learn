// 17. 곱셈 전용 함수 정의해보기
fn multiplication(num_a: i64, num_b: i64) -> i64 {
    num_a * num_b
}
fn main(){
    let num_a = 152;
    let num_b = 100;
    let result = multiplication(num_a, num_b);
    println!("{} x {} = {}", num_a, num_b, result)
}
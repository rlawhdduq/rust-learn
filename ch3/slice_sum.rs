// 8. 러스트의 슬라이스 타입
// 배열과 문자열 외에 벡터에 대해서도 슬라이스를 얻을 수 있다.

fn sum_slice(items: &[i64]) -> i64{
    let mut total = 0;
    for item in items {
        total += item;
    };
    total
}

fn main(){
    // 배열 초기화
    let a = [1,2,3,4,5,6,7,8,9,10];
    println!("a={}", sum_slice(&a[..]));

    // 벡터 초기화
    let b = vec![1,2,3,4,5,6,7,8,9,10];
    println!("b={}", sum_slice(&b[..]));
}
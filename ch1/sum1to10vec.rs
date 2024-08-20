// 24. 포문 다루기 - 가변 배열 - 벡터(맛보기만 하고 다음챕터에서 상세히 다룸)
fn main(){
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut total = 0;
    for i in nums {
        total += i;
    }
    println!("{}", total);
}
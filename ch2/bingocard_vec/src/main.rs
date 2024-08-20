// 37. 러스트로 빙고카드 만들기 - 벡터 사용
// python에서는 random 모듈의 shuffle 기능이 있었지만, 러스트에선 표준 메서드 중 배열을 섞는 기능은 없다
// 허나 rand 크레이트에는 배열을 섞는 기능이 존재해서 해당 크레이트를 사용하여 배열을 섞는다.

// 배열을 섞기 위한 rand 크레이트 선언
use rand::seq::SliceRandom;

fn main() {
    // 1에서 75까지의 숫자로 이루어진 배열을 생성
    let mut nums = vec![];
    for i in 1..=75 { nums.push(i); }

    // 섞기
    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);

    // 카드 표시
    for i in 0..25 {
        if i == 12 {
            print!(" *,");
        } else {
            print!("{:3},", nums[i]);
        }
        if i % 5 == 4 { println!(""); }
    }
}

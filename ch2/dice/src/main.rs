// 30. 크레이트를 사용해서 러스트로 주사위 프로그램 만들기
// rand 크레이트의 Rng 이용
use rand::Rng;

fn main() {
    // 난수 생성기 준비
    let mut rng = rand::thread_rng();
    // 5번 반복 수행
    for i in 1..=5 {
        // 1~6사이의 난수를 생성
        let dice = rng.gen_range(1..=6);
        println!("{}", dice);
    }
}


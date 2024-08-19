// 13. 러스트로 거스름돈 조합 계산
// 어느 가게의 계산 카운터에 500원짜리 10개, 100원짜리 3개, 50원짜리 10개가 있다.
// 잔돈으로 3950원을 거슬러 줘야 할 경우 나올 수 있는 모든 조합을 나열하시오
fn main(){
    let price = 3950;
    for obak in 0..11 {
        for bak in 0..4 {
            for osib in 0..11 {
                let total_price = (obak*500) + (bak*100) + (osib*50);
                if total_price == price {
                    println!("오백원 {}개, 백원 {}개, 십원 {}개", obak, bak, osib);
                }
            }
        }
    }
}
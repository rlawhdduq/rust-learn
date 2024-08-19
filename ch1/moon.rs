fn main(){
    // 3. 지구에서 달 까지의 거리는 384,400km다.
    // 3. 지구에서 달 까지 80km/h 속도의 자동차와 300km/h 속도의 KTX로 간다면 각 이동 수단은 며칠이 걸리는지 계산하시오
    let moon = 384400.0;
    let car = 80.0;
    let ktx = 300.0;

    println!("자동차 = {}일", (moon / car / 24.0));
    println!("기차 = {}일", (moon / ktx / 24.0));
}
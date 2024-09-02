// 9. 구조체를 이용한 BMI 계산
// 키와 몸무게를 구조체로 만든 뒤 BMI 측정 하는 프로그램
struct Body {
    height: f64,
    weight: f64,
}

fn main(){
    let hong = Body {
        height: 165.0,
        weight: 80.0,
    };
    let lim = Body {
        height: 170.0,
        weight: 65.0,
    };

    // 함수 호출
    println!("홍길동 = {:.1}", calc_bmi(&hong));
    println!("임꺽정 = {:.1}", calc_bmi(&lim));
}

fn calc_bmi(body: &Body) -> f64 {
    let h = body.height / 100.0;
    body.weight / h.powf(2.0)
}
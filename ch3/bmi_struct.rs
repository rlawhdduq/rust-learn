// 10. 러스트에서 BMI 판정표를 이용해 BMI 판정
struct BmiRange {
    min: f64,               // min 이상
    max: f64,               // max 미만
    label: &'static str,    // 판정
}

fn main(){
    // 키와 몸무게 입력
    let height = input("키(cm) : ");
    let weight = input("몸무게 : ");

    // BMI 계산
    let height = height / 100.0;
    let bmi = weight / height.powf(2.0);

    // 비만도 판정표를 벡터 타입으로 생성
    let bmi_list = vec![
        BmiRange {
            min: 0.0,
            max: 18.5,
            label: "저체중",
        },
        BmiRange {
            min: 18.5,
            max: 23.0,
            label: "정상",
        },
        BmiRange {
            min: 23.0,
            max: 25.0,
            label: "비만 전 단계",
        },
        BmiRange {
            min: 25.0,
            max: 30.0,
            label: "1단계 비만",
        },
        BmiRange {
            min: 30.0,
            max: 35.0,
            label: "2단계 비만",
        },
        BmiRange {
            min: 35.0,
            max: 99.0,
            label: "3단계 비만",
        },
    ];

    // 비만도 판단
    let mut result = "계산불가";
    for range in bmi_list {
        if range.min <= bmi && bmi < range.max {
            result = range.label;
            break;
        }
    }

    println!("BMI = {1:.1}, 비만도 = {0}", result, bmi);
}

fn input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("입력 에러");
    s.trim().parse().expect("숫자 변환 에러")
}
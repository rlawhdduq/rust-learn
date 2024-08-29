// 6. 함수를 호출해서 인수를 변경하는 방법

// 아래는 예시이다.
// // 함수 정의
// fn 함수 이름(변수명: &mut 타입) {
//     // 실젯값을 얻음
//     let value = *변수명;
//     // 변수를 갱신
//     *변수명 = 새로운 값;
// }

// // 함수 호출
// 함수(&mut 변수);

// 인수의 값에 2를 곱해 반환하는 함수
fn x2(arg: &mut i32) {
    *arg = *arg * 2;
}

fn main(){
    let mut num = 50;
    println!("{}", num);
    x2(&mut num); // 인수에 2가 곱해진다
    println!("{}", num);
}
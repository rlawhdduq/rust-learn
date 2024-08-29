// 6. (부록) println! 매크로 사용 방법 정리
// 다음과 같이 인수에 이름을 붙여서 대입하는 것도 가능하다.

fn main(){
    let year = 2023;
    let month = 12;
    let day = 1;
    println!("{yy}년 {mm}월 {dd}일",
            dd=day, mm=month, yy=year);
}

// 12. 섀도잉을 사용하면 가변변수를 이용하는 부분도 줄일 수 있다.
fn main(){
    // 섀도잉을 이용하지 않음
    {
        let mut v = 300; // v를 가변 변수로 선언
        v = v + 5;
        println!("{}", v);
    }
    // 섀도잉을 이용
    {
        let v = 300; // v는 불변변수
        let v = v + 5;
        println!("{}", v);
    }
}
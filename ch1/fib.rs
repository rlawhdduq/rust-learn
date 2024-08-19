// 8. 피보나치 수열 구하기
// mut 는 가변 변수로 지정하겠다는 키워드이다
// 러스트 변수는 기본적으로 '불변'이다. ( 타 프로그래밍 언어들과 다른점 )
fn main(){
    let mut a = 1;
    let mut b = 1;
    println!("{}", a);
    println!("{}", b);

    for _ in 0..30 {
        println!("{}", (a+b));
        let tmp = a;
        a = b;
        b = tmp + a;
    }
}
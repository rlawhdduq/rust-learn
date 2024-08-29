// 7. 러스트의 튜플, 배열, 슬라이스
// 튜플을 이용하면 다양한 타입을 가지는 몇 개의 타입을 모아 하나의 복합 타입으로 만들 수 있다. 러스트에서도 물론 이 튜플을 이용할 수 있다.
// 튜플은 값을 괄호로 감싸 만들 수 있다.

// -> 구조체라고 보면 되는건가?

fn main(){
    // 튜플 만들기
    let banana = ("바나나", 300);
    let apple  = ("사과", 200);

    // 튜플을 참조해 합계 금액 구하기
    let total = banana.1 + apple.1;

    // 튜플의 내용 표시
    print_tuple(&banana);
    print_tuple(&apple);
    println!("합계는 {}원 입니다.", total);

}

fn print_tuple(item: &(&str, i64)){
    println!("{}를 {}원에 구입", item.0, item.1);
}
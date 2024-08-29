// 7. 러스트의 튜플, 배열, 슬라이스
// 자주 이용하는 튜플이라면 구조체로 정의하는 것도 좋은 방법이다. 튜플을 구조체로 정의하는 것은 다음과 같다
/**
struct 구조체명 (타입1, 타입2, ...);
 */

struct Item(String, i64);

fn main(){
    // 튜플 만들기
    let banana = Item("바나나".to_string(), 300);
    let apple  = Item("사과".to_string(), 200);
    let mango  = Item("망고".to_string(), 500);

    // Item을 벡터에 추가
    let items = vec![banana, apple, mango];

    // 합계 금액 구하기
    let total = print_end_sum_items(&items);
    println!("합계는 {}원 입니다.", total);

}

// 튜플을 표시하는 함수
fn print_tuple(item: &Item){
    println!("{}를 {}원에 구입", item.0, item.1);
}

// 아이템을 순서대로 표시하고 합계 금액 구하기
fn print_end_sum_items(items: &Vec<Item>) -> i64 {
    let mut total = 0;
    for item in items {
        print_tuple(&item);
        total += item.1;
    }

    total // 합계 금액 반환
}

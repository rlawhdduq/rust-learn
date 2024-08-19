// 12. 제품 가격 비교해보기
// 어떤 컴퓨터 업체의 PC 가격은 98만원이다. 
// A 쇼핑몰에서는 배송비가 12,000원이고 20% 할인된 가격으로 판매하고 있다.
// B 쇼핑몰에서는 배송료가 무료지만 10% 할인된 가격으로 팔고 있다.
// 두 쇼핑몰 중 어느 쇼핑몰이 싼지 계산하시오
fn main(){
    let pc_price = 980000.0;
    let shop_a_sale = 0.2;
    let shop_a_deliver_fee = 12000.0;
    let shop_b_sale = 0.1;

    println!("A업체 가격 : {}", (pc_price - (pc_price * shop_a_sale)) + shop_a_deliver_fee);
    println!("B업체 가격 : {}", (pc_price - (pc_price * shop_b_sale)));
    if (pc_price-(pc_price * shop_a_sale)) + shop_a_deliver_fee < (pc_price-(pc_price * shop_b_sale)) {
        println!("A 업체가 더 쌉니다");
    }
    else {
        println!("B 업체가 더 쌉니다");
    }
}
// 25. 타언어에서 사용하는 삼항연산자 흉내내기 ( 러스트에는 삼항연산자가 없다 )
fn main(){
    let n = 5;
    let check_even_odd = if n % 2 == 0 { "짝수" } else { "홀수" } ;
    println!("{}", check_even_odd);
}
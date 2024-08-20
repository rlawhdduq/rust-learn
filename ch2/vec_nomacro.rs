// 35. 매크로를 쓰지않고 벡터를 선언 할 수도 있다.
// 매크로 쓰는게 훨씬 간단하다 원리만 알아두자
fn main(){
    let mut nums = Vec::new();
    nums.push(1);
    nums.push(2);
    nums.push(3);

    println!("{:?}", nums);
}
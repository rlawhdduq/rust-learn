// 2. 소유자가 유효 범위에서 벗어나면 파기
fn main(){
    // 블록
    {
        let s1 = String::from("재능은 한계가 있지만 노력엔 한계가 없다");
        println!("{}", s1);
    }
    // 블록을 벗어나면 s1은 자동으로 호출되는 drop 함수에 의해 파기된다
}
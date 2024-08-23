// 41. Result타입을 반환하는 parse 메서드 다루기2
// 에러 처리를 생략하고싶다면 expect나 unwrap 메서드를 이용하자
// 단, 해당 메서드들은 프로그램이 바로 강제종료된다는 것을 명심하고 적재적소에 필요한 메서드를 채택해서 사용하자.
fn main(){
    let s = "365";
    let i: i32 = s.parse().unwrap();
    println!("{}", i);
}
// 44. 16진수와 8진수와 2진수를 지정할 수 있다.
fn main(){
    let v1 = 0xFF; // 16진수로 255를 지정
    let v2 = 0o655; // 8진수로 429를 지정
    let v3 = 0b1101_0101; // 2진수로 213을 지정
    println!("{}, {}, {}", v1, v2, v3);
}
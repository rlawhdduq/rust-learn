// 16. 러스트로 시저 암호 만들기
fn encrypt(txt: &str, shift: i16) -> String{
    // 'A'와 'Z'의 문자코드를 i16 타입으로 획득
    let code_a = 'A' as i16;
    let code_z = 'Z' as i16;
    // 결과를 대입할 변수를 선언
    let mut enc_txt = String::new();
    for idx in txt.chars() {
        let mut txt_code = idx as i16;
        if code_a < txt_code && txt_code < code_z {
            txt_code += shift;
        }
        enc_txt.push((txt_code as u8) as char);
    }
    return enc_txt;
}

// 19. 시저 함수 조금 더 우아하게 만들기
fn encrypt2(txt: &str, shift: i16) -> String {
    let is_az = |n| 'Z' > n && n > 'A';
    let conv = |n| ((n+shift) as u8) as char;
    let enc1 = |n| if is_az(n) {conv(n as i16)} else {n};

    txt.chars().map(|n| enc1(n)).collect()
}
fn main(){
    // 16
    println!("16");
    let txt = "I LOVE RUST";
    let shift = 3;
    let enc = encrypt(txt, shift);
    let dec = encrypt(&enc, -shift);
    println!("{} => {} => {}", txt, enc, dec);

    // 19
    println!("19");
    let txt = "I LOVE RUST";
    let shift = 3;
    let enc = encrypt2(txt, shift);
    let dec = encrypt2(&enc, -shift);
    println!("{} => {} => {}", txt, enc, dec);
}
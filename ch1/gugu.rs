// 6. 러스트로 구구단 만들기
// fn main(){
//     for y in 1..10 {
//         for x in 1..10 {
//             print!("{:3}", (y*x));
//             if x < 9 {
//                 print!(",")
//             }
//         }
//         println!("")
//     }
// }

// 마지막 행엔 쉼표(,) 제거하기
fn main(){
    for y in 1..10 {
        let s = (1..10)
                .map(|x| format!("{:3}", (x * y)))
                .collect::<Vec<String>>().join(",");
            println!("{}", s)
    }
}
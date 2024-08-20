// 36. 다양한 벡터 타입 사용해보기
// 벡터는 Vec<T> 라는 데이터 타입이다. 여기서 T는 제네릭이라는 러스트의 편리한 기능 중 하나
// Vec<u32>, Vec<&str>, 등의 타입을 지정해서 벡터 타입을 만들 수 있다.

fn main(){
    // u32타입의 벡터 생성
    let a_vec: Vec<u32> = vec![100, 200, 300];
    let b_vec: Vec<&str> = vec!["김김", "종종", "엽엽"];

    println!("{:?}", a_vec);
    println!("{:?}", b_vec);

    for i in a_vec {
        println!("{}", i);
    }
    for i in b_vec {
        println!("{}", i);
    }
}
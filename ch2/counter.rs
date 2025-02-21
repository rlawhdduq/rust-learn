// 44. 인기 투표 집계 - rust
// HashMap을 사용하기 위한 선언
use std::collections::HashMap;

// 투표 데이터를 상수로 선언 -> 상수 선언 시에는 타입을 반드시 명시해줘야한다!
const V_DATA: &str = "C,C,A,A,A,B,C,C,B,B,B,C,B,C,B,A,C,C,B,C,C,C";

fn main(){
    // 집계용 HashMap 생성
    let mut c_map = HashMap::new();

    // HashMap을 0으로 초기화
    c_map.insert("A", 0);
    c_map.insert("B", 0);
    c_map.insert("C", 0);

    // 투표 데이터 집계
    for i in V_DATA.split(',') {
        c_map.insert(i, c_map[i]+1);
    }

    // 집계 후 결과 표시
    for k in ["A", "B", "C"] {
        println!("{}:{:>2}", k, c_map[k]);
    }
}
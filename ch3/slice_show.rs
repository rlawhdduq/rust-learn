// 8. 러스트의 슬라이스 타입
// 슬라이스란 배열이나 벡터, 문자열 같은 데이터 타입의 요소 중 일부를 참조하기 위한 것이다.
// 기본적으로 참조자이므로 슬라이스에는 '소유권이 없다'
// beep라는 문자열에서 앞 3글자의 슬라이스를 얻는다면 bee라는 문자열의 참조자를 얻게 된다.
// 배열에 대한 슬라이스라면 배열의 n번째부터 m번째 까지의 요소에 대한 참조자를 표현할 수 있다
// 슬라이스는 '&변수명[n..m]'형태로 이용한다.

fn main(){
    // 배열 초기화
    let a:[i32;6] = [0, 1, 2, 3, 4, 5];

    // 배열 a의 앞 3개 요소의 슬라이스를 얻음
    let a_slice = &a[0..3];
    // let a_slice = &a[..3];
    println!("{:?}", a_slice); // [0, 1, 2]

    // a에서 4번째부터 5번째까지의 슬라이스를 얻음
    println!("{:?}", &a[3..5]);// [3, 4]

    // a에서 4번째 이후의 슬라이스를 얻음
    println!("{:?}", &a[3..6]);// [3, 4, 5]
    // println!("{:?}", &a[3..]);// [3, 4, 5]
}
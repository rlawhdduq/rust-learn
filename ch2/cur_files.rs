// 58. 러스트의 파일 경로 표현
// 현재 디렉터리의 파일 목록을 표시하는 프로그램

use std::fs;

fn main(){
    // 파일 목록 취득
    let files = fs::read_dir(".").expect("올바르지 않은 경로입니다.");
    for ent in files {
        // 목록을 하나씩 처리
        let entry = ent.unwrap();
        
        // PathBuf 오브젝트 얻기
        let path = entry.path();

        // 파일 이름 출력
        let name = path.to_str().unwrap_or("올바르지 않은 파일 이름입니다.");
        println!("{}", name);
    }
}
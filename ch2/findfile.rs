// 57. 러스트로 파일 재귀 검색하기
use std::{env, path};

fn main(){
    // 명령줄 인수 확인
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("findfile (path) (keyword)");
        return;
    }

    // 명령줄 인수 값 얻기
    let target_dir = &args[1];
    let keyword = &args[2];

    // PathBuf로 변환
    let target = path::PathBuf::from(target_dir);
    findfile(&target, keyword);
}

fn findfile(target: &path::PathBuf, keyword: &str) {
    // 파일 목록을 취득
    let files = target.read_dir().expect("존재하지 않는 경로");
    for dir_entry in files {
        // PathBuf로 경로 취득
        let path = dir_entry.unwrap().path();
        // println!("path = {:?}", path);
        // 디렉터리라면 자신을 다시 호출해 파일을 검색
        if path.is_dir() {
            findfile(&path, keyword);
            continue;
        }

        // 파일 이름을 문자열로 변환
        let name = path.file_name().unwrap()
                        .to_string_lossy();
        // println!("name = {}", name);
        // 검색어(파일 이름)를 포함하지는 확인
        if None == name.find(keyword) { continue; }

        // 검색어를 포함하는 경로 표시
        println!("{}", path.to_string_lossy());
    }
}
# 52. 파이썬으로 2개의 텍스트 파일 비교하기

# 파일 이름 지정
afile = "./fizzbuzz_python.txt"
bfile = "./fizzbuzz_rust.txt"

# 파일 내용을 읽어들임
with open(afile, "r", encoding="utf-8") as fp:
    astr = fp.read()
with open(bfile, "r", encoding="utf-8") as fp:
    bstr = fp.read()

# 불필요한 공백 삭제(trim)
astr = astr.strip()
bstr = bstr.strip()

# 비교
if astr == bstr :
    print("ok")
else :
    print("ng")
# 16. 파이썬으로 시저 암호 만들기
def encrypt(txt, shift):
    enc_txt = ""
    code_a = ord("A")
    code_z = ord("Z")
    for idx in range(0, len(txt)):
        txt_ord = ord(txt[idx])
        if code_a < txt_ord < code_z:
            txt_ord = txt_ord+shift
        enc_txt += chr(txt_ord)
    return enc_txt

# 17. 시저 함수 조금 더 우아하게 만들기
def encrypt2(txt, shift):
    conv = lambda n: chr(ord(n)+shift)
    enc1 = lambda n: conv(n) if 'A' <= n <= 'Z' else n
    return ''.join([enc1(n) for n in txt])
originTxt = "I LOVE RUST"
shift = 3
encTxt = encrypt2(originTxt, shift)
decTxt = encrypt2(encTxt, -shift)
print("origin = {}".format(originTxt))
print("encrypt = {}".format(encTxt))
print("decrypt = {}".format(decTxt))




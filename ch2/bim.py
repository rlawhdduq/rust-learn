# 38. 비만도 진단 도구
# 키와 체중 입력
height_cm = float(input("키 : "))
weight = float(input("몸무게 : "))

# BMI 계산
height = height_cm / 100.0
bmi = weight / (height ** 2)
print("BIM = {:.1f}".format(bmi))

# 진단결과 표시
if bmi < 18.5:
    print("저체중")
elif bmi < 23:
    print("정상")
elif bmi < 25:
    print("비만 전 단계")
elif bmi < 30:
    print("1단계 비만")
elif bmi < 35:
    print("2단계 비만")
else:
    print("3단계 비만")
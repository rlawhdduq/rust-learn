# 10. 판정 기준을 만들어 비만도 판정하기
# 키와 몸무게 입력
height_cm = float(input("키 : "))
weight = float(input("몸무게 : "))

# BMI 계산
height = height_cm / 100.0
bmi = weight / (height ** 2)

# 비만도 판정표 - 판정용 사전 타입 리스트
bmi_list = [
    {"min":0, "max":18.5, "label":"저체중"},
    {"min":18.5, "max":23, "label":"정상"},
    {"min":23, "max":25, "label":"비만 전 단계"},
    {"min":25, "max":30, "label":"1단계 비만"},
    {"min":30, "max":35, "label":"2단계 비만"},
    {"min":35, "max":99, "label":"3단계 비만"},
]

# 판정
result = "판정불가"
for range in bmi_list:
    if range["min"] <= bmi < range["max"]:
        result = range["label"]

print("BMI = {:1f}, 비만도 = {}".format(bmi, result))
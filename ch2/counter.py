# 44. 인기 투표 집계
# 투표 데이터
V_DATA = "C,C,A,A,A,B,C,C,B,B,B,C,B,C,B,A,C,C,B,C,C,C"

# 집계용 사전 타입 데이터 초기화
c_dic = {"A":0, "B":0, "C":0}

for i in V_DATA.split(",") :
    c_dic[i] += 1

# 집계 후 결과 표시
for key in ["A", "B", "C"] :
    print("{}: {:2d}".format(key, c_dic[key]))

# 집계 후 결과 표시(내버전)
for key in c_dic.keys():
    print("{}:{:2d}".format(key, c_dic[key]))
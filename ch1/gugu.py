# 6. 파이썬으로 구구단 만들기
# for y in range(1, 10):
#     for x in range(1, 10):
#         print("{:3},".format(y * x), end="")
#     print("")

# 행의 마지막 값엔 쉼표(,) 표시하지 않기
for y in range(1, 10):
    a = ["{:3}".format(y * x) for x in range(1, 10)]
    print(",".join(a))
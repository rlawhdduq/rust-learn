# 3. 지구에서 달 까지의 거리는 384,400km다.
# 3. 지구에서 달 까지 80km/h 속도의 자동차와 300km/h 속도의 KTX로 간다면 각 이동 수단은 며칠이 걸리는지 계산하시오
dest = 384400
car = 80
ktx = 300
print("자동차 = {}일".format(( dest / car / 24 )))
print("기차 = {}일".format(( dest / ktx / 24 )))
# 13. 파이썬으로 거스름돈 조합 계산
# 어느 가게의 계산 카운터에 500원짜리 10개, 100원짜리 3개, 50원짜리 10개가 있다.
# 잔돈으로 3950원을 거슬러 줘야 할 경우 나올 수 있는 모든 조합을 나열하시오

# 거스름돈
price = 3950
obak_cnt = 0
bak_cnt = 0
osib = 0

for obak in range(0, 11):
    for bak in range(0, 4):
        for osib in range(0, 11):
            total_price = (obak*500) + (bak*100) + (osib*50)
            if total_price == price :
                print("오백원 {}개, 백원 {}개, 오십원 {}개".format(obak, bak, osib))

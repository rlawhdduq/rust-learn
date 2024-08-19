# 4. 1부터 100까지의 숫자를 출력한다
# 단, 3의 배수 일 땐 fizz 5의 배수일 때는 buzz를 출력하시오
# 3과 5의 공배수 일 땐 fizzbuzz를 출력하시오

for i in range(1, 101):
    if i%3==0 and i%5==0:
        i = 'FizzBuzz'
    elif i%3==0 :
        i = 'Fizz'
    elif i%5==0 :
        i = 'Buzz'
    print(i)
# 20. 소수인지 판단하는 함수
def is_prime(n):
    for i in range(2, n):
        print("{}, {}".format(n, i))
        if n % i == 0:
            return False
    return True
    
# count만큼 소수를 생성
def get_primes(count):
    res = []
    i = 2
    while len(res) < count :
        if is_prime(i):
            res.append(i)
        i += 1
    return res

print(get_primes(10))
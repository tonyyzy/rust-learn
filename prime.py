import math


def is_prime(num):
    for i in range(2, round(math.sqrt(num))):
        if num % i == 0:
            return False
    return True


for i in range(1000001):
    is_prime(i)

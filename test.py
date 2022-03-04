import time

delays = []
wait_time = 0.1

for x in range(10):
    start = time.time()
    time.sleep(wait_time)
    result = time.time() - start
    print(result)
    delays.append(result)

print(f'Average : {sum(delays)/len(delays)}')

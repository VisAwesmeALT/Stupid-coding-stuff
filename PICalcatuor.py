# Made By DrKel
# Please Credit DrKel (Or VisAwesme or Vaughn R)

import psutil
import time
import math

def calculate_pi():
    pi = 0
    k = 0
    while True:
        pi += ((4.0 * (-1)**k) / (2*k + 1))
        k += 1
        yield pi

def monitor_cpu_usage():
    return psutil.cpu_percent(interval=1)

def main():
    generator = calculate_pi()
    cpu_usage_threshold = 80.0

    while True:
        pi_estimate = next(generator)
        print(f"Current estimate of pi: {pi_estimate}")
        
        cpu_usage = monitor_cpu_usage()
        print(f"CPU usage: {cpu_usage}%")
        
        if cpu_usage > cpu_usage_threshold:
            print("CPU usage exceeded threshold. Terminating the script.")
            break

if __name__ == "__main__":
    main()

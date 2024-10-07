#include <iostream>
#include <vector>

/**
 * You are given N numbers. Check if there's a subset of them, with the sum
 * equal to target value S.
 * Being N <= 20.
 */

using namespace std;

int subset_sum(int a[], int n, int s) {
  for (int mask = 0; mask < (1 << n); mask++) {
    long long sum = 0;

    for (int i = 0; i < n; i++) {
      if (mask & (1 << i)) {
        sum += a[i];
      }
    }

    if (sum == s) {
      puts("YES");
      return 0;
    }
  }
  puts("NO");
  return 1;
}

int main(void) {
  int a[] = {1, 4, 10, 9, 1, 12, 15, 13};

  subset_sum(a, sizeof(a) / sizeof(a[0]), 2);

  return 0;
}

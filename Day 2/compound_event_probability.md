### Task:
There are **3** urns labeled **X**, **Y**, and **Z**.
- Urn **X** contains **4** red balls and **3** black balls.
- Urn **Y** contains **5** red balls and **4** black balls.
- Urn **Z** contains **4** red balls and **4** black balls.

One ball is drawn from each of the **3** urns. What is the probability that, of the **3** balls drawn, **2** are red and **1** is black?

### Solution:
Total cases:
- Urn 1: `4 Red + 3 Black` = **7** Balls
- Urn 2: `5 Red + 4 Black` = **9** Balls
- Urn 3: `4 Red + 4 Black` = **8** Balls

Therefore, Total `7 * 9 * 8` = **504** Cases.

Now, We draw **3** balls, one from each Urns. Out of which, **2** should be Red and **1** Black.

Therefore, Number of possibilities:
1. RRB: 1 Red from Urn 1, 1 Red from Urn 2, 1 Black from Urn 3.
2. RBR: 1 Red from Urn 1, 1 Black from Urn 2, 1 Red from Urn 3.
3. BRR: 1 Black from Urn 1, 1 Red from Urn 2, 1 Red from Urn 3.

- **RRB**:

  We can choose any of the **4** Red balls from Urn 1, any of the **5** Red balls from Urn 2 and any of the **4** Black balls from Urn 3.
  
  Therefore, we can have `4 * 5 * 4` = **80** favourable cases.
  
- **RBR**: 

  We can choose any of the **4** Red balls from Urn 1, any of the **4** Black balls from Urn 2 and any of the **4** Red balls from Urn 3.
  
  Therefore, we can have `4 * 4 * 4` = **64** favourable cases.
  
- **BRR**:

  We can choose any of the **3** Black balls from Urn 1, any of the **5** Red balls from Urn 2 and any of the **4** Red balls from Urn 3.
  
  Therefore, we can have `3 * 5 * 4` = **60** favourable cases.
  
In total, we can have `80 + 64 + 60` = **204** favourable cases.

Therefore, final probability for the given task will be `204 / 504` = **`17/42`**

---
Reference: [HackerRank Statistics Day 2: Probabilities](http://coders-errand.com/hackerrank-statistics-day-2-probabilities/)

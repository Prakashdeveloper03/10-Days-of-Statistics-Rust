### Task:
In a single toss of **2** fair (_evenly-weighted_) six-sided dice, find the probability that the values rolled by each die will be different and the two dice have a sum of **6**.

### Solution:

- Sample space (Ω): `{1, 2, 3, 4, 5, 6} * {1, 2, 3, 4, 5, 6}`
- Event A := "Sum of both dice is **6**" = `{(1, 5), (2, 4), (3, 3), (4, 2), (5, 1)}`
- `|A| = 5`, but condition is that the values rolled by each die should be different, therefore, `(3, 3)` will not count.
  
  Therefore, `|A| = |A| - {(3, 3)} = |A| - 1 = 4`
- P(A) = `|A| / |Ω|` = `4 / 36` = `1 / 9`

Probabilty that the sum of both differently rolled dice will be **6** is **`1/9`**.

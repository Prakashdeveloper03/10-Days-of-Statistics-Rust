### Task:
You draw **2** cards from a standard **52**-card deck without replacing them. What is the probability that both cards are of the same suit?

### Solution:
Let's consider we already drew **1** card from the deck. Now, we have **51** cards remaining to choose the second card from.

- Total cases of choosing one random card from remaining **51** cards is: **_`C(51, 1)`_** = **51**

- Favourable cases:

  We want our second card to have the same suit as our first card.
  
  Therefore, after choosing first card, we have only **12** cards remaining in that suit to choose our second card from.
  
  Total favourable cases: **_`C(12, 1)`_** = **12**
  
Probability that both cards are of the same suit: **_`C(12, 1) / C(51, 1)`_** = **`12/51`**

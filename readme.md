## Problem 13. Roman to integer

https://leetcode.com/problems/roman-to-integer/

If next Roman digit makes a pair with previous, just subtract double value of previous digit from result.

## Problem 14. Longest common prefix

https://leetcode.com/problems/longest-common-prefix/

It's ok to compare current string with previous to find a longest prefix

## Problem 1. Two sum

https://leetcode.com/problems/two-sum/

Easy because there are always two numbers in the answer.

Just store visited number in the hash table with index and check
if it contains difference between current and target.

## Problem 3. Longest unique substring

Solution is suboptimal because of map mutations.

Store each symbol with index in map. If repeated index is occurred, remove all pairs from the map with smaller index.

Could be optimized by storing last index of symbol in hash map.
Instead of clearing map, remember uniq substring index and
to calculate length count all pairs with greater index.

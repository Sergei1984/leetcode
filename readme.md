## Problem 13. Roman to integer (Easy)

https://leetcode.com/problems/roman-to-integer/

If next Roman digit makes a pair with previous, just subtract double value of previous digit from result.

## Problem 14. Longest common prefix (Easy)

https://leetcode.com/problems/longest-common-prefix/

It's ok to compare current string with previous to find a longest prefix

## Problem 1. Two sum (Easy)

https://leetcode.com/problems/two-sum/

Easy because there are always two numbers in the answer.

Just store visited number in the hash table with index and check
if it contains difference between current and target.

## Problem 3. Longest unique substring (Easy)

Solution is suboptimal because of map mutations.

Store each symbol with index in map. If repeated index is occurred, remove all pairs from the map with smaller index.

Could be optimized by storing last index of symbol in hash map.
Instead of clearing map, remember uniq substring index and
to calculate length count all pairs with greater index.

## Problem 5. Longest palindrome (Medium)

At first find a pairs of the same symbols and store them sorted in binary heap.
Then start checking from widest to narrows and return first found palindrome.

Solution is unexpectedly slow, not sure why.
Probably it should be better to just scan the string and
try expanding compare symbols left and right around certain symbol
(but need to handle palindromes of both types `aa` and `aba`).

## Problem 2. Add two numbers (Medium)

Add two numbers represented as linked list of digits.

Pretty easy algorithmically but hard to operate with linked lists in Rust.
Solved via recursive algorithm but not sure if it would be such easy to
implement such algorithm using loops.

## Problem 8. String to integer (Medium)

Convert string containing some garbage characters to integer

Pretty easy but need to implement a lot of conditions.
The most complex part is to determine overflowing but easy to do in Rust.

## Problem 10. Regular expression matching (Hard)

Fucking hard really. A lot of conditions and cases needs to be encounted.
Most of time I spent to realize how algorithm must behave in different situations.

Main stuck: cases like `.*a`. For input `abcaa` it's not possible to determine end of wildcard without testing all variants.

Solution could be slightly optimized by preventing all-variants check in some cases.
For example

- for patterns like `a*b` we could omit checking at all
- for patterns like `a*aaa` where `a` occurred `n` times at the end we could replace all-variants checking with
  checking that `a` occurred at least `n` times.

## Problem 7. Reverse integer (Medium)

Done without converting source int to string.
Most challenging was to determine last digit of number for numbers near max and min integer,
because method used (taking reminder and subtracting previous reminder) requires multiplier
equal to i32.MAX \* 10 (which wouldn't work and causes overflow).

Work around that by checking actual overflow and testing number with if,
because there are two possible first digits (1 or 2) for near end numbers.

## Problem 12. Integer to Roman (Medium)

Pretty straightforward but requires decent amount of code.
Seems convenient algorithm - split to 10-power bases and convert using given set of rules.

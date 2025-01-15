# ATAD_problems

## The Power Sum

The function uses a helper function to explore all possible combinations of integers starting from 1.

For each integer, it decides whether to:
-Include the current integer (subtract its n-th power from the target and continue).
-Exclude the current integer (move to the next integer without subtracting).

The recursion stops when:
-The sum of powers exceeds the target value (x).
-The sum of powers exactly equals the target value (x).

The total count of valid combinations is returned.

## Crossword Puzzle

Converts the input crossword grid into a mutable 2D character array and splits the input words into a list.

Sorts the words by length in descending order to prioritize placing longer words first.

Uses a recursive backtracking approach to try placing each word in the grid, either horizontally or vertically, at every valid position.

Ensures that words can only be placed in positions where they fit and align with existing characters in the grid.

Tracks positions where letters are placed, enabling the function to "unplace" words if they lead to dead ends.

Continues the process until all words are placed, returning the completed crossword.

## Non-Divisible Subset problem

The function calculates the remainders of each element in the input array when divided by k and stores their frequencies in a count array.

The subset size starts at zero. A special case is handled for numbers whose remainder is 0, as only one such number can be included in the subset.

The function iterates over possible remainders from 1 to k/2:
-If a remainder r is equal to k−r (a special case when k is even), only one number with that remainder can be included.
-Otherwise, for each remainder r, the function selects the larger frequency between r and k−r to maximize the subset size.

The calculated subset size is returned.

## Bigger is Greater

Starting from the rightmost character, find the first character (from the end) that is smaller than the character immediately after it. This is the "pivot." If no such pivot exists, the input string is already the largest permutation, and the function returns "no answer".

From the rightmost end, find the smallest character that is larger than the pivot. This character will be swapped with the pivot to create the next larger permutation.

Swap the pivot with the identified character to form a partially rearranged string.

Reverse the characters following the pivot to ensure the result is the smallest lexicographical permutation greater than the original string.

Convert the modified character array back into a string and return it.

## Lily's Homework

This helper function calculates the number of swaps required to transform the input array into a target order. It uses a hashmap to keep track of the current positions of elements for efficient swapping. Each element is moved to its correct position in the target order, incrementing the swap count.

The input array is sorted in ascending order to calculate the swaps needed for the ascending arrangement.

The sorted array is reversed to calculate the swaps needed for the descending arrangement.

The function returns the smaller of the two swap counts, as either order (ascending or descending) is acceptable for making the array beautiful.

## Time in Words

The function uses a predefined list of words for numbers from 0 to 29, allowing it to convert numeric values into their word equivalents.

-Exact Hours:
If the minutes (m) are 0, the function returns the format "<hour> o' clock" (e.g., "five o' clock").

-For minutes between 1 and 30:
Special cases like 15 minutes are expressed as "quarter past".
30 minutes is expressed as "half past".
Singular 1 minute is expressed as "one minute past".
Other values are expressed as "<minutes> minutes past <hour>".

-For minutes greater than 30, the function calculates the remaining minutes to the next hour and adjusts the hour accordingly:
Special cases like 15 minutes remaining are expressed as "quarter to".
Singular 1 minute is expressed as "one minute to".
Other values are expressed as "<minutes> minutes to <next hour>".

## Staircase

The function loops through each row from 1 to n.

In each row:
-It prints the necessary number of spaces to align the hashes correctly.
-It prints the required number of # symbols to form that level of the staircase.

The final result is a staircase pattern where the number of spaces decreases and the number of # symbols increases as you move down each row.

## Angry professor

The function counts how many students are on time (arrival time <= 0) and compares it to the threshold k.

If the number of on-time students is less than k, the class is canceled, and the function returns "YES". Otherwise, it returns "NO".

## Birthday candles

Determines how many candles have the maximum height on a birthday cake.

It finds the tallest candles and counts how many candles have that maximum height.

## Grading students

For each grade:
-If the grade is greater than or equal to 38, calculate the next multiple of 5.
-If the difference between the grade and this multiple is less than 3, round the grade up.
-If the grade is less than 38, leave it as is.

## Migratory birds

It creates a HashMap to count how many times each bird type appears in the input array (arr).
For each bird type in arr, it updates the count in the HashMap.

It iterates through the HashMap to find the bird type with the highest count.
If two or more bird types have the same count, it picks the one with the smallest bird ID.

The function returns the bird type that appears the most times, or the smallest bird type ID if there is a tie.

## Time conversion

Parse hours, minutes, seconds, and period (AM/PM) from the input string.

If AM and hour is 12, set hour to 0 (midnight).
If PM and hour is not 12, add 12 to the hour (convert to 24-hour time).

Format the hour, minutes, and seconds into a 24-hour time string (hh:mm:ss).

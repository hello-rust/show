from collections import defaultdict

words = [word for word in open("/usr/share/dict/words")]

word_lengths = defaultdict(set)
for word in words:
    word_lengths[len(word)].add(word)
print(word_lengths)


def quicksort(items):
    if not items:
        return []
    pivot = items[0]
    smaller = [i for i in items if i < pivot]
    bigger = [i for i in items[1:] if i >= pivot]
    return quicksort(smaller) + [pivot] + quicksort(bigger)


print(quicksort([6, 2, 7, 2, 3, 4]))

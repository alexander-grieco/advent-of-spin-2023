# Test how many present can be in Santa's sack, when he has to travel the least
# Please note, that we will a different set of numbers when validating your submission
POST https://challenge-3-ohc228ww.fermyon.app
{
    "place": "North Pole",
    "characters": ["Santa Claus", "The Grinch", "a pingvin"],
    "objects": ["A spoon", "Two presents", "Palm tree"]
}

HTTP 200
[Asserts]
jsonpath "$.story" != null

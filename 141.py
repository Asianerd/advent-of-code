raw_data_string = """
PHVCVBFHCVPFKBNHKNBO

HK -> F
VN -> S
NB -> F
HF -> B
CK -> N
VP -> B
HO -> P
NH -> N
CC -> N
FC -> P
OK -> S
OO -> P
ON -> C
VF -> B
NN -> O
KS -> P
FK -> K
HB -> V
SH -> O
OB -> K
PB -> V
BO -> O
NV -> K
CV -> H
PH -> H
KO -> B
BC -> B
KC -> B
SO -> P
CF -> V
VS -> F
OV -> N
NS -> K
KV -> O
OP -> O
HH -> C
FB -> S
CO -> K
SB -> K
SN -> V
OF -> F
BN -> F
CP -> C
NC -> H
VH -> S
HV -> V
NF -> B
SS -> K
FO -> F
VO -> H
KK -> C
PF -> V
OS -> F
OC -> H
SK -> V
FF -> H
PK -> N
PC -> O
SP -> B
CB -> B
CH -> H
FN -> V
SV -> O
SC -> P
NP -> B
BB -> S
PV -> S
VB -> P
SF -> H
VC -> O
HN -> V
BF -> O
NO -> O
HP -> N
VV -> K
HS -> P
FH -> N
KB -> F
KF -> B
PN -> K
KH -> K
CN -> S
PP -> O
BP -> O
OH -> B
FS -> O
BK -> B
PO -> V
CS -> C
BV -> N
KP -> O
KN -> B
VK -> F
HC -> O
BH -> B
FP -> H
NK -> V
BS -> C
FV -> F
PS -> P
"""

polymer = ''
matchings = {}

polymer_section = True
for x in raw_data_string.strip().split("\n"):
    if not x:
        polymer_section = False
        continue

    if polymer_section:
        polymer = x
    else:
        _match_data = x.split(" -> ")
        matchings[_match_data[0]] = _match_data[1]

for iteration in range(10):
    matches = []
    for index in range(len(polymer)):
        pair = polymer[index:index + 2]
        if len(pair) < 2:
            continue
        matches.append(matchings[pair])
    modified = list(polymer)
    for index, match in enumerate(matches):
        modified.insert((index * 2) + 1, match)
    polymer = ''.join(modified)

occurance = {}
for x in polymer:
    if x in occurance:
        occurance[x] += 1
    else:
        occurance[x] = 1

print(occurance)
least = 999999999999999999
most = -99
for x in occurance:
    if occurance[x] < least:
        least = occurance[x]
    if occurance[x] > most:
        most = occurance[x]

minimum = most - least
print(minimum)

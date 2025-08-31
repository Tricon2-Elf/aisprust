import re
import sys

with open(sys.argv[1], "r") as f:
    text = f.read()

pattern = re.compile(r"((?:#\[.*?\]\s*)+)(pub struct\s+(\w+)[^{]*\{.*?\})", re.DOTALL)
matches = pattern.findall(text)

structs = [(m[2], m[0] + m[1]) for m in matches]
structs.sort(key=lambda x: x[0])

for _, block in structs:
    print(block.strip(), end="\n\n")


for name, block in structs:
    print("//", name.strip())

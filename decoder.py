import sys

# HZ decoder for whois description
for line in sys.stdin:
    line = list(line.strip()[2:-2].encode('utf-8'))
    for i in range(len(line)):
        line[i] = line[i] | 0x80
    print(bytes(line).decode('gb2312', 'replace'))

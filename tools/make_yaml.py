import yaml

def strip(lis):
    if lis[-1] == '':
        return strip(lis[:-1])
    else:
        return lis

with open("syscalls.html") as input, open("c syscalls.yaml", "w") as output:
    i = 0
    for line in input.readlines():
        line = line.strip().replace("<tr><td>", "").replace("</td></tr>", "")
        args = line.split("</td><td>")
        args = strip(args)
        args[1] = args[1].replace("sys_", "")
        yaml.dump({args[1]: { "n": int(args[0]), "args": args[2:]}}, output)
from pycparser import c_parser, c_ast
import os
import argparse 
import sys
import yaml

def get_cwd(filename=""):
    val = get_cwd.value
    if val != None:
        return val
    else:
        get_cwd.value = os.path.dirname(os.path.realpath(__file__))+"/"+filename
        return get_cwd(filename)

get_cwd.value = None

def parse(args: str):
    parser = c_parser.CParser()
    args = args.replace("ssize_t", "long").replace("size_t", "unsigned long")
    node = parser.parse("{};".format(args))
    
    # print(type(node))
    if(not isinstance(node, c_ast.FileAST)):
        raise Exception("Gay")
    dec = node.ext[0]
    return transform(dec)

class RType:
    def __init__(self, name):
        self.name = name
        self.uname = name,
    
    def unsigned_name(self, uname):
        self.uname = uname
        return self
    
    def get(self, uname):
        if uname:
            return self.uname
        else:
            return self.name

tp_table = {
    "int": RType("i32").unsigned_name("u32"),
    "long": RType("i64").unsigned_name("u64"),
    "char": RType("i8").unsigned_name("u8"),
    "void": RType("i8").unsigned_name("u8")
}

def transform(decl):
    def const(d):
        if "const" in d.quals:
            return "const"
        else:
            return "mut"
    
    tp = type(decl)
    # print(tp)
    if tp == c_ast.Decl:
        return [decl.name + ":"] + transform(decl.type)
    if tp == c_ast.ArrayDecl:
        return ["*", "const"] + transform(decl.type)
    elif tp == c_ast.PtrDecl:
        return ["*", const(decl)] + transform(decl.type)
    elif tp == c_ast.TypeDecl:
        tp = decl.type.names
        return [tp_table.get(tp[-1]).get(len(tp) > 1 and tp[1] == "unsigned")]

regs = [
    "rdi",
    "rsi",
    "rdx",
    "r10",
    "r8",
    "r9",
]

class SyscallVariant:
    def __init__(self, name: str, args: [[str]], number, stname: str):
        self.name = name
        self.args = args
        self.stname = stname
        self.number = number
    
    def _normalized(self):
        return list(map(" ".join, self.args))
    
    def enum(self):
        fmt_args = ""
        if len(self.args) > 0:
            fmt_args = "({})".format(self.name)
        
        return ["{}{},\n".format(self.name, fmt_args)]
    
    def struct(self):
        fmt_args = []
        if len(self.args) > 0:
            fmt_args.append(f"pub struct {self.name} {{\n")
            for arg in self._normalized():
                fmt_args.append(f"\t pub {arg},\n")
            fmt_args.append("}\n")
            return fmt_args
        else:
            return [""]

    def from_regs(self):
        fmt_args = ""
        if len(self.args) > 0:
            arg_name = lambda i: self.args[i][0]
            args = lambda i: " ".join(self.args[i][1:])

            fmt_args = ["{} {} as {}".format(arg_name(i), regs[i], args(i)) for i, x in enumerate(self.args)]
            fmt_args = ", ".join(fmt_args)
            fmt_args = "({}{{{}}})".format(self.name, fmt_args)
        
        return [f"{self.number} => {self.stname}::{self.name}{fmt_args},\n"]

def into_rust(stname:str, call_list:[str], output:str):
    with open(get_cwd("c syscalls.yaml")) as y:
        failed = []
        calls = []
        for call, args in yaml.full_load(y).items():
            if not call in call_list:
                continue
            try:
                rust_args = [parse(arg) for arg in args["args"]]
                call_name = ''.join(map(str.capitalize, call.split('_')))

                calls.append(SyscallVariant(call_name, rust_args, args["n"], stname))

            except Exception as e:
                failed.append(call, e)
        
        if len(failed) > 0:
            print("Could not parse syscalls: {}".format(failed))

        lines = {"structs": [], "variants": [], "from_regs": []}
        for call in calls:
            lines["structs"].extend(call.struct())
            lines["variants"].extend(call.enum())
            lines["from_regs"].extend(call.from_regs())

        with open(output, "w") as out:
            out.writelines(lines["structs"])

            out.write(f"pub enum {stname} {{\n")
            out.writelines(["\t"+line for line in lines["variants"]])
            out.write("}\n\n")

            out.writelines([f"impl {stname} {{\n",
            f"\tpub fn from_regs(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> Option<{stname}> {{\n",
            "\t\t let call = match rax {\n"])
            out.writelines(["\t\t\t" + line for line in lines["from_regs"]])
            out.writelines(["\t\t\t_ => return None\n",
            "\t\t};\n",
            "\treturn Some(call);\n",
            "\t}\n",
            "}\n"])



wanted_syscalls = [
    "read",
    "write",
    "open",
    "close",
    # "writev",
    "fork",
    "vfork",
    "execve",
    "rename",
    "mkdir",
    "creat",
    "unlink",
    "symlink",
    "openat",
    "mkdirat",
    "renameat",    
]

parser = argparse.ArgumentParser()
parser.add_argument("stname", type=str)
parser.add_argument("out", type=str)
parser.add_argument("calls", nargs="+")
args = parser.parse_args()

with open("out.log", "w") as debug:
    debug.write("Out file: {}\nStruct name: {}\nCalls: {}\n".format(args.out, args.stname, args.calls))
    try:
        into_rust(stname=args.stname, call_list=args.calls, output=args.out)
    except Exception as e:
        debug.write(str(e))
    
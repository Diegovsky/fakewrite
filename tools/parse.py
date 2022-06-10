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

def parse_type(args: str):
    parser = c_parser.CParser()
    args = args.replace("ssize_t", "long").replace("size_t", "unsigned long")
    node = parser.parse("{};".format(args))
    
    assert isinstance(node, c_ast.FileAST)

    dec = node.ext[0]
    return transform(dec)

class RType:
    def __init__(self, name):
        self.name = name
        self.uname = name
    
    def unsigned_name(self, uname):
        self.uname = uname
        return self
    
    def get(self, uname) -> str:
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

''' This function is really lacking and still doesn't do mutable and consts right.
    I'm planing on rewriting everything here, making my own c function parameter parser im ruby,
     as can be seen on ctor.rb, but I have zero idea of how parsing works, so...
'''
def transform(decl: c_ast.Node) -> list[str]:
    def const(d):
        if "const" in d.quals:
            return "const"
        else:
            return "mut"
    
    if isinstance(decl, c_ast.Decl):
        return [decl.name + ":"] + transform(decl.type)
    if isinstance(decl, c_ast.ArrayDecl):
        return ["*", "const"] + transform(decl.type)
    elif isinstance(decl, c_ast.PtrDecl):
        return ["*", const(decl)] + transform(decl.type)
    elif isinstance(decl, c_ast.TypeDecl):
        names: list[str] = decl.type.names
        return [tp_table[names[-1]].get(len(names) >= 2 and names[1] == "unsigned")]
    else:
        return []

regs = [
    "rdi",
    "rsi",
    "rdx",
    "r10",
    "r8",
    "r9",
]

class SyscallVariant:
    def __init__(self, name: str, args: list[list[str]], number: str, stname: str):
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

def into_rust(yaml_path: str, stname:str, call_list: list[str], output:str):
    with open(get_cwd(yaml_path)) as y:
        failed: list[tuple[str, Exception]] = []
        calls: list[SyscallVariant] = []
        for func_name, args in yaml.full_load(y).items():
            if not func_name in call_list:
                continue
            try:
                print(func_name, args)
                rust_args = [parse_type(arg) for arg in args["args"]]
                # Rough snake_case to CamelCase operation.
                call_name = ''.join(map(str.capitalize, func_name.split('_')))

                calls.append(SyscallVariant(call_name, rust_args, args["n"], stname))

            except Exception as e:
                failed.append((func_name, e))
        
        if len(failed) > 0:
            print("Could not parse syscalls: {}".format(failed))

        lines = {"structs": [], "variants": [], "from_regs": []}
        for func_name in calls:
            lines["structs"].extend(func_name.struct())
            lines["variants"].extend(func_name.enum())
            lines["from_regs"].extend(func_name.from_regs())

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
parser.add_argument("yaml", type=str, metavar='syscalls.yaml')
parser.add_argument("stname", type=str, metavar='struct name')
parser.add_argument("out", type=str, metavar='output file')
parser.add_argument("calls", nargs="?", default=wanted_syscalls)
args = parser.parse_args()

with open('./out.log', "w") as debug:
    debug.write("Out file: {}\nStruct name: {}\nCalls: {}\n".format(args.out, args.stname, args.calls))
    stdout = sys.stdout
    stderr = sys.stderr
    sys.stdout = debug
    sys.stderr = debug
    try:
        into_rust(yaml_path=args.yaml, stname=args.stname, call_list=args.calls, output=args.out)
    except Exception as e:
        print('Something unexpected happened, see out.log', file=stderr)
        debug.write(str(e))
    

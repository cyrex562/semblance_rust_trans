# run the rust refactoring operations on files in the src dir
import errno
import os
import sys
import plumbum
from plumbum import local
from plumbum.cmd import ls, c2rust, rustc, cargo, rm
Command = plumbum.machines.LocalCommand

def die(emsg, ecode=1):
    """
    log fatal error and exist with specified error code
    :param emsg:
    :param ecode:
    :return:
    """
    print(f"error: {emsg}")
    quit(ecode)


# lifted from c2rust scripts
def get_cmd_or_die(cmd: str) -> Command:
    try:
        plumbum.local[cmd]
    except plumbum.CommandNotFound:
        die(f"{cmd} not in path", errno.ENOENT)

refactor = get_cmd_or_die("c2rust-refactor")


def run_refactor(command, tgt_file, mode='inplace'):
    """
    full_args = ['-r', mode] + args + [
            '--', 'src/lib.rs', '--crate-type=dylib',
            '--crate-name=json_c',
            '-L{rust_libdir}/rustlib/{triple}/lib/'.format(
                rust_libdir=get_rust_toolchain_libpath(),
                triple=get_host_triplet())]
    :param args:
    :param mode:
    :return:
    """
    full_args = ['refactor', '-r', mode, command, '--cargo', tgt_file]
    c2rust[full_args]()

REFACTORINGS = [
    'link_incomplete_types',
    'fix_unused_unsafe',
    'fold_let_assign',
    'link_funcs',
    'reconstruct_for_range',
    'remove_redundant_casts',
    'remove_redundant_let_types',
    'remove_unused_labels',
    'struct_assign_to_update',
    'struct_merge_updates',
    ## last
    # 'reorganize definitions'
]

def run() -> int:
    curr_dir = local.cwd
    src_dir = os.path.join(str(local.cwd), 'src')
    files = ls(src_dir).split('\n')

    for f in files:
        x = os.path.join(src_dir, f)
        print(f"refactoring f{x}")
        # run refactor command
        for r in REFACTORINGS:
            print(f"running {r} operation")
            run_refactor(r, x)
    return 0


if __name__ == "__main__":
    sys.exit(run())

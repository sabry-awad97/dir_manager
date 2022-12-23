import argparse
import os
import pathlib

import pyfiglet


def list_dir_contents(filepath):
    files = os.listdir(filepath)
    detailed_files = []
    for file in files:
        file_path = pathlib.Path(filepath).joinpath(file)
        stat = file_path.stat()
        detailed_files.append(
            {
                "filename": file,
                "size": stat.st_size,
                "created_at": stat.st_ctime,
            }
        )

    # Find the maximum length of the 'filename', 'size', and 'created_at' fields
    n = max(len(record['filename'])
            for record in detailed_files)
    s = max(len(str(record['size']))
            for record in detailed_files)
    c = max(len(str(record['created_at']))
            for record in detailed_files)

    name = 'filename'.ljust(n)
    size = 'size'.rjust(s)
    created = 'created_at'.center(c)
    print(f'{name:<{n}}  {size:>{s}}  {created:^{c}}')
    for record in detailed_files:
        name = record['filename'].ljust(n)
        size = str(record['size']).rjust(s)
        created = str(record['created_at']).center(c)
        print(f'{name:<{n}}  {size:>{s}}  {created:^{c}}')


def create_dir(filepath):
    if not os.path.exists(filepath):
        os.mkdir(filepath)
        print("The directory has been created successfully")


def create_file(filepath):
    pathlib.Path(filepath).touch()
    print("An empty file has been created")


def parse_args():
    parser = argparse.ArgumentParser(
        description="An example CLI for managing a directory"
    )
    parser.add_argument(
        "-l", "--ls", type=str, help="List directory contents", default="."
    )
    parser.add_argument(
        "-m", "--mkdir", type=str, help="Create a directory", default=""
    )
    parser.add_argument(
        "-t", "--touch", type=str, help="Create a file", default=""
    )
    args = parser.parse_args()
    return args


def main():
    args = parse_args()
    pyfiglet.print_figlet("Dir Manager")

    if args.ls:
        list_dir_contents(args.ls)
    if args.mkdir:
        create_dir(args.mkdir)
    if args.touch:
        create_file(args.touch)


if __name__ == "__main__":
    main()

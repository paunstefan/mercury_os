import struct
import sys
import os

initrd_header_fmt = 'B'
initrd_file_fmt = '<64sQQ'
file_header_size = struct.calcsize(initrd_file_fmt)


def main():
    if len(sys.argv) != 2:
        sys.exit(f"Usage: {sys.argv[0]} [directory]")

    files = os.listdir(sys.argv[1])
    paths = [sys.argv[1] + "/"  + p for p in files]
    print(paths)

    no_files = len(files)
    header = struct.pack(initrd_header_fmt, no_files)

    data = b""
    data += header
    offset = 1 + file_header_size * no_files

    for i in range(no_files):
        file_size = os.path.getsize(paths[i])
        print(file_size)
        name = bytes(files[i], 'utf-8')
        data += struct.pack(initrd_file_fmt, name, file_size, offset)
        offset += file_size

    for i in range(no_files):
        infile = open(paths[i], "rb")
        contents = infile.read()
        data += contents
        infile.close()


    with open("iso/modules/initrd", "wb") as f:
        f.write(data)

if __name__ == "__main__":
    main()
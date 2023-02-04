import struct

initrd_header_fmt = 'B'
initrd_file_fmt = '<64sQQ'
file_header_size = struct.calcsize(initrd_file_fmt)

no_files = 2;

# f1
f1_content = b"Hello from the initrd";

# f2
f2_content = b"another file"

files = [f1_content, f2_content]

data = struct.pack(initrd_file_fmt, b"file1.txt", len(f1_content), 0)

header = struct.pack(initrd_header_fmt, no_files)

data = b""

data += header

offset = 1 + file_header_size * no_files
for i in range(len(files)):
    name = b"file" + bytes(str(i), 'utf-8') + b".txt"
    data += struct.pack(initrd_file_fmt, name, len(f1_content), offset)
    offset += len(f1_content)

for i in range(len(files)):
    data += files[i]

print(f"size {len(data)}")
print(data)

# Writing to file
with open("iso/modules/initrd", "wb") as f:
    # Writing data to a file
    f.write(data)
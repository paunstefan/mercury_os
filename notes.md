# Notes

## Rust target installation

```bash
rustup target add x86_64-unknown-none
```

## Boot process

1) Enable PAE in CR4
2) Set CR3 so it points to the PML4
3) Enable IA-32e mode
4) Enable paging
5) Load GDT (for long mode, it will just be flat memory)
6) Long jump to 64bit code


## Links

* <https://www.pagetable.com/?p=14>
* <https://forum.osdev.org/viewtopic.php?t=11093>
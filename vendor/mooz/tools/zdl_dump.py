#!/usr/bin/env python3
"""Inspect an MS-70CDR ZDL's ELF image (sections, symbols, data words).

  zdl_dump.py ANLGDLY                     # sections and symbols
  zdl_dump.py ANLGDLY 0x80000430 20       # 20 words from an address: hex, float, int
  zdl_dump.py ANLGDLY .const              # a whole section

ELF images come from zoom-ms70-cdr/artifacts/analysis/2.10/elf/<NAME>.ZDL.elf.
"""
import struct, sys, os
ELF_DIR = '/Users/georgy/Projects/zoom-ms70-cdr/artifacts/analysis/2.10/elf'

def load(name):
    path = name if os.path.exists(name) else os.path.join(ELF_DIR, name.upper().removesuffix('.ZDL') + '.ZDL.elf')
    if not os.path.exists(path):
        path = os.path.join(ELF_DIR, name + '.ZDL.elf')
    data = open(path, 'rb').read()
    shoff, = struct.unpack_from('<I', data, 0x20)
    shentsize, shnum, shstrndx = struct.unpack_from('<HHH', data, 0x2e)
    sections = []
    for i in range(shnum):
        n, t, f, addr, off, size, link, info, align, es = struct.unpack_from('<10I', data, shoff + i * shentsize)
        sections.append(dict(name_off=n, type=t, flags=f, addr=addr, off=off, size=size, link=link, entsize=es))
    strtab = sections[shstrndx]
    for s in sections:
        end = data.index(b'\0', strtab['off'] + s['name_off'])
        s['name'] = data[strtab['off'] + s['name_off']:end].decode()
    symbols = []
    for s in sections:
        if s['type'] == 2:  # SYMTAB
            names = sections[s['link']]
            for j in range(s['size'] // 16):
                n, value, size, info, other, shndx = struct.unpack_from('<IIIBBH', data, s['off'] + j * 16)
                end = data.index(b'\0', names['off'] + n)
                nm = data[names['off'] + n:end].decode(errors='replace')
                if nm and not nm.startswith('$'):
                    symbols.append((value, size, info & 15, nm))
    return data, sections, sorted(set(symbols))

def words_at(data, sections, addr, count):
    for s in sections:
        if s['type'] != 8 and s['addr'] <= addr < s['addr'] + s['size'] and s['size']:
            off = s['off'] + addr - s['addr']
            n = min(count, (s['addr'] + s['size'] - addr) // 4)
            return s['name'], struct.unpack_from('<%dI' % n, data, off)
    return None, ()

def main():
    data, sections, symbols = load(sys.argv[1])
    if len(sys.argv) == 2:
        for s in sections:
            if s['name']:
                print(f"{s['name']:<20} addr {s['addr']:08x} size {s['size']:6x} type {s['type']}")
        print()
        for value, size, kind, name in symbols:
            print(f"{value:08x} {size:6x} {'FUNC' if kind == 2 else 'OBJ ' if kind == 1 else kind} {name}")
        return
    target = sys.argv[2]
    if target.startswith('.'):
        s = next(s for s in sections if s['name'] == target)
        addr, count = s['addr'], s['size'] // 4
    else:
        addr = int(target, 0)
        count = int(sys.argv[3], 0) if len(sys.argv) > 3 else 16
    name, ws = words_at(data, sections, addr, count)
    if name is None:
        print('address not in a loaded section'); return
    print(f'section {name}')
    for i, w in enumerate(ws):
        f, = struct.unpack('<f', struct.pack('<I', w))
        i32, = struct.unpack('<i', struct.pack('<I', w))
        print(f"{addr + 4 * i:08x}: {w:08x}  f={f:<14.8g} i={i32:<12d} q31={i32 / 2**31:.8f}")

main()

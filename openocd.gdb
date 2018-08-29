target remote :3333

# print demangled symbols
set print asm-demangle on

# detect unhandled exceptions and hard faults
break DefaultHandler
break UserHardFault

monitor arm semihosting enable

load
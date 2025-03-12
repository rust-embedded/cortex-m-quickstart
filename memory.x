MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* 注意：1 K = 1 KiB = 1024 字节 */
  /* TODO Adjust these memory regions to match your device memory layout */
  /* 待办：根据你的设备内存布局调整这些内存区域 */
  /* These values correspond to the LM3S6965, one of the few devices QEMU can emulate */
  /* 这些值对应于 LM3S6965，这是 QEMU 能够模拟的少数设备之一 */
  /* FLASH 内存区域，起始地址为 0x00000000，长度为 256 KiB */
  FLASH : ORIGIN = 0x00000000, LENGTH = 256K  
  /* RAM 内存区域，起始地址为 0x20000000，长度为 64 KiB */
  RAM : ORIGIN = 0x20000000, LENGTH = 64K     
  
}

/* This is where the call stack will be allocated. */
/* 这是调用栈将被分配的地方。 */
/* The stack is of the full descending type. */
/* 栈是满递减类型的。 */
/* You may want to use this variable to locate the call stack and static
   variables in different memory regions. Below is shown the default value */
/* 你可能希望使用这个变量来将调用栈和静态变量定位到不同的内存区域。下面显示的是默认值。 */
/* _stack_start = ORIGIN(RAM) + LENGTH(RAM); */
/* _stack_start = RAM 的起始地址 + RAM 的长度 */

/* You can use this symbol to customize the location of the .text section */
/* 你可以使用这个符号来自定义 .text 段的位置 */
/* If omitted the .text section will be placed right after the .vector_table
   section */
/* 如果省略，.text 段将被放置在 .vector_table 段之后 */
/* This is required only on microcontrollers that store some configuration right
   after the vector table */
/* 这仅在那些在向量表之后存储一些配置的微控制器上是必需的 */
/* _stext = ORIGIN(FLASH) + 0x400; */
/* _stext = FLASH 的起始地址 + 0x400 */

/* Example of putting non-initialized variables into custom RAM locations. */
/* 将未初始化的变量放入自定义 RAM 位置的示例。 */
/* This assumes you have defined a region RAM2 above, and in the Rust
   sources added the attribute `#[link_section = ".ram2bss"]` to the data
   you want to place there. */
/* 这假设你已经在上面定义了一个 RAM2 区域，并且在 Rust 源代码中为你想要放置在那里的数据添加了属性 `#[link_section = ".ram2bss"]`。 */
/* Note that the section will not be zero-initialized by the runtime! */
/* 注意：该段不会被运行时初始化为零！ */
/* SECTIONS {
     .ram2bss (NOLOAD) : ALIGN(4) {
       *(.ram2bss);
       . = ALIGN(4);
     } > RAM2
   } INSERT AFTER .bss;
*/

MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  /* TODO Adjust these memory regions to match your device memory layout */
  FLASH : ORIGIN = 0xBAAAAAAD, LENGTH = 0K
  RAM : ORIGIN = 0xBAAAAAAD, LENGTH = 0K
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

/* You can use this symbol to customize the location of the .text section */
/* If omitted the .text section will be placed right after the .vector_table
   section */
/* This is required only on some microcontrollers that store some configuration
   right after the vector table */
/* _stext = ORIGIN(FLASH) + 0x400; */

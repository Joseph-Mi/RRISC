/* STM32G474VET6 Memory Layout */
/* Flash: 512KB, SRAM: 128KB (80KB + 16KB + 32KB CCM) */

MEMORY
{
  /* Flash memory for program code */
  FLASH : ORIGIN = 0x08000000, LENGTH = 512K
  
  /* Main SRAM for data and heap */
  RAM : ORIGIN = 0x20000000, LENGTH = 80K
  
  /* SRAM2 for additional data if needed */
  RAM2 : ORIGIN = 0x20014000, LENGTH = 16K
  
  /* CCM SRAM for high-performance data (DMA not accessible) */
  CCMRAM : ORIGIN = 0x10000000, LENGTH = 32K
}

/* Stack size (8KB) */
_stack_size = 8K;

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

target remote :3333
monitor arm semihosting enable
# if using ITM
# monitor tpiu config internal itm.fifo uart off 8000000
# monitor itm port 0 on
load
tbreak start
continue

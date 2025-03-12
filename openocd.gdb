# Connect to the remote target using port 3333
# 连接到远程目标，使用端口 3333
target extended-remote :3333

# Print demangled symbols for better readability
# 打印反混淆后的符号名称，便于阅读
set print asm-demangle on

# Set backtrace limit to 32 to prevent infinite backtrace loops
# 设置回溯限制为 32，防止无限回溯循环
set backtrace limit 32

# Detect unhandled exceptions, hard faults, and panics
# 检测未处理的异常、硬件故障和 panic
break DefaultHandler  # Set a breakpoint at DefaultHandler
                      # 在 DefaultHandler 处设置断点
break HardFault       # Set a breakpoint at HardFault
                      # 在 HardFault 处设置断点
break rust_begin_unwind  # Set a breakpoint at rust_begin_unwind
                         # 在 rust_begin_unwind 处设置断点

# If you want to execute a few steps immediately after a panic message is printed,
# uncomment the following lines and adjust the number of steps as needed for your panic handler
# 如果需要在 panic 消息打印后立即执行后续操作，可以取消注释以下代码
# 并根据 panic 处理程序的实际情况调整 next 的步数
# commands $bpnum
# next 4
# end

# Try to set a breakpoint at the user entry point (main function)
# Note: Due to inlining, the main function might be optimized away
# 尝试在用户入口点 main 函数处设置断点
# 注意：由于内联优化，main 函数可能会被优化掉，因此不一定能成功
break main

# Enable ARM semihosting to allow interaction between the debugger and the target device
# 启用 ARM 半主机模式，允许调试器与目标设备进行交互
monitor arm semihosting enable

# To capture ITM data and send it to a file (itm.fifo), uncomment the following lines
# Note: The microcontroller's SWO pin must be connected to the programmer's SWO pin
# 8000000 must match the core clock frequency
# 如果需要捕获 ITM 数据并将其发送到文件 itm.fifo，可以取消注释以下代码
# 注意：单片机的 SWO 引脚必须连接到编程器的 SWO 引脚
# 8000000 必须与核心时钟频率匹配
# monitor tpiu config internal itm.txt uart off 8000000

# Alternatively, configure the microcontroller's SWO pin to output UART-compatible data (8N1)
# 8000000 must match the core clock frequency
# 2000000 is the frequency of the SWO pin
# 或者：将单片机的 SWO 引脚配置为与 UART (8N1) 兼容的输出
# 8000000 必须与核心时钟频率匹配
# 2000000 是 SWO 引脚的输出频率
# monitor tpiu config external uart off 8000000 2000000

# Enable ITM port 0 for debugging output
# 启用 ITM 端口 0，用于调试输出
# monitor itm port 0 on

# Load the program onto the target device
# 将程序加载到目标设备的内存中
load

# Start the process but immediately halt the processor
# 启动程序，但立即暂停处理器
stepi
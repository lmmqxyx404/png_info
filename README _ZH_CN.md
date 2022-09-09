# png_info
这个项目是一个初学者所作的项目，功能主要是实现了在png图片中进行文字信息隐写。

# dependency
crc: 计算crc校验码
structopt：处理命令行参数

# modules
文件模块主要分为五大部分
## png
png 文件由固定文件头和一系列的 chunk 数据共同组成。可以用16进制编辑器打开
## chunk_type
根据公开的png文件标准，创建符合要求的chunk_type。

## chunk
创建不同的chunk



## args
使用enum创建不同的指令

## commands
具体的指令功能实现


## 通过wintun学习UDP协议

此分支是开坑的基础，实现了一个从3层的tun设备上接收ipv4包，然后解析ipv4包、UDP包的场景

## 平台

Windows10

## 构建

`cargo build --release`


## tun网卡的设置（代码里面设置，不需要自己动手）

42是我这边初始化出来的id，不同的机器可能不一样

```bash
netsh interface ip set interface 42 metric=255
netsh interface ip set address 42 static 10.28.13.2/24 gateway=10.28.13.1
netsh interface ip add route 10.28.13.2/24 42 10.28.13.1
```

## 测试

通过nc命令（没有的自己写udp去连也行）

`nc -u 10.28.13.100 4321`


这里端口随意，进去后随便输点东西即可，然后tun这边就可以将ip/udp协议解析出来，并打印一些输出，像下面这样

```text
C:\Users\pysrc>nc -u 10.28.13.100 4321
Hello World !
```

```text
dst ip 10.28.13.100 port: 4321 recv: Hello World !
```


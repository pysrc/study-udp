use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::channel,
    Arc,
};
mod protocol;

fn main() {
    // 加载wintun
    let wintun =
        unsafe { wintun::load_from_path("wintun.dll") }.expect("Failed to load wintun dll");

    // 打开或创建一个虚拟网卡装置
    let adapter = match wintun::Adapter::open(&wintun, "Demo") {
        Ok(a) => a,
        Err(_) => wintun::Adapter::create(&wintun, "Example", "Demo", None)
            .expect("Failed to create wintun adapter!"),
    };

    // 设置虚拟网卡信息
    // ip = 10.28.13.2 mask = 255.255.255.0 gateway = 10.28.13.1
    let index = adapter.get_adapter_index().unwrap();
    let set_metric = format!("netsh interface ip set interface {} metric=255", index);
    let set_gateway = format!(
        "netsh interface ip set address {} static 10.28.13.2/24 gateway=10.28.13.1",
        index
    );

    // 打印输出
    println!("{}", set_metric);
    println!("{}", set_gateway);

    // 执行网卡初始化命令
    std::process::Command::new("cmd")
        .arg("/C")
        .arg(set_metric)
        .output()
        .unwrap();
    std::process::Command::new("cmd")
        .arg("/C")
        .arg(set_gateway)
        .output()
        .unwrap();

    // 添加设置测试路由，所有10.28.13.2/24子网下的流量都走10.28.13.1网关（也就是我们上面创建的虚拟网卡）
    let set_route = format!(
        "netsh interface ip add route 10.28.13.2/24 {} 10.28.13.1",
        index
    );

    // 打印输出
    println!("{}", set_route);

    // 执行添加路由命令
    std::process::Command::new("cmd")
        .arg("/C")
        .arg(set_route)
        .output()
        .unwrap();

    // 开启tun会话
    let session = Arc::new(adapter.start_session(wintun::MAX_RING_CAPACITY).unwrap());

    // 读的一端
    let reader_session = session.clone();

    // 写的一端
    let writer_session = session.clone();

    // 发送结构体
    struct Resp {
        data: Vec<u8>,
        src_ip: [u8; 4],
        src_port: u16,
        dst_ip: [u8; 4],
        dst_port: u16,
    }

    let (tx, rx) = channel::<Resp>();

    // 全局运行标志
    static RUNNING: AtomicBool = AtomicBool::new(true);

    let reader = std::thread::spawn(move || {
        while RUNNING.load(Ordering::Relaxed) {
            match reader_session.receive_blocking() {
                Ok(packet) => {
                    // 收到ip包数据
                    let bytes = packet.bytes();

                    // 解析判断是否ipv4的udp协议（RFC 790）
                    if protocol::ipv4_version(bytes) == 4
                        && protocol::ipv4_protocol(bytes) == 17
                        && protocol::ipv4_dst_addr(bytes) == &[10, 28, 13, 100]
                    {
                        // 打印udp数据
                        let dstip = protocol::ipv4_dst_addr(bytes);
                        let udp_pack = protocol::ipv4_data(bytes);
                        println!(
                            "dst ip {}.{}.{}.{} port: {} recv: {}",
                            dstip[0],
                            dstip[1],
                            dstip[2],
                            dstip[3],
                            protocol::udp_dst_port(udp_pack),
                            String::from_utf8_lossy(protocol::udp_data(udp_pack))
                        );

                        // 发送响应包
                        let mut resp_data = "Recv: ".as_bytes().to_vec();
                        resp_data.extend_from_slice(protocol::udp_data(udp_pack));

                        // 发送/接收方相反
                        let srcip = protocol::ipv4_src_addr(bytes);
                        let resp = Resp {
                            data: resp_data,
                            src_ip: [dstip[0], dstip[1], dstip[2], dstip[3]],
                            src_port: protocol::udp_dst_port(udp_pack),
                            dst_ip: [srcip[0], srcip[1], srcip[2], srcip[3]],
                            dst_port: protocol::udp_src_port(udp_pack),
                        };
                        tx.send(resp).unwrap();
                    }
                }
                Err(_) => unreachable!(),
            }
        }
    });

    let writer = std::thread::spawn(move || {
        while RUNNING.load(Ordering::Relaxed) {
            let resp = rx.recv().unwrap();
            // 申请发送buffer
            let mut write_pack = writer_session
                .allocate_send_packet(28 + resp.data.len() as u16)
                .unwrap();
            let mut resp_pack = write_pack.bytes_mut();

            // 构建响应IP包
            protocol::ipv4_udp_build(
                &mut resp_pack,
                &resp.src_ip,
                resp.src_port,
                &resp.dst_ip,
                resp.dst_port,
                &resp.data,
            );

            // 发送响应包
            writer_session.send_packet(write_pack);
        }
    });

    // 按任意键回车结束
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line);
    println!("Shutting down session");
    RUNNING.store(false, Ordering::Relaxed);
    session.shutdown();
    let _ = reader.join();
    let _ = writer.join();
}

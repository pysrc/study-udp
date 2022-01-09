use std::sync::{
    atomic::{AtomicBool, Ordering},
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

    // 全局运行标志
    static RUNNING: AtomicBool = AtomicBool::new(true);

    let reader = std::thread::spawn(move || {
        while RUNNING.load(Ordering::Relaxed) {
            match reader_session.receive_blocking() {
                Ok(packet) => {
                    // 收到ip包数据
                    let bytes = packet.bytes();

                    // 解析判断是否ipv4的udp协议（RFC 790）
                    if protocol::ipv4_version(bytes) == 4 && protocol::ipv4_protocol(bytes) == 17 {
                        // 打印udp数据
                        let ipv4 = protocol::ipv4_dst_addr(bytes);
                        let udp_pack = protocol::ipv4_data(bytes);
                        println!(
                            "dst ip {}.{}.{}.{} port: {} recv: {}",
                            ipv4[0],
                            ipv4[1],
                            ipv4[2],
                            ipv4[3],
                            protocol::udp_dst_port(udp_pack),
                            String::from_utf8_lossy(protocol::udp_data(udp_pack))
                        );
                    }
                }
                Err(_) => unreachable!(),
            }
        }
    });

    // 按任意键回车结束
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line);
    println!("Shutting down session");
    RUNNING.store(false, Ordering::Relaxed);
    session.shutdown();
    let _ = reader.join();
}

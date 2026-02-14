# Draft — โครงร่างมาตรฐานสำหรับบทความใน Rust Feed

โฟล์เดอร์นี้เก็บ **Spec & Template** สำหรับบทความในแต่ละหมวดของ Rust Feed เพื่อให้ทุกบทความมีโครงสร้าง คุณภาพ และทิศทางที่สอดคล้องกันครับ

---

## รายการ Spec ตามหมวดหมู่

| หมวดหมู่ | ไฟล์ Spec | รหัสบทความ | คำอธิบาย |
| --- | --- | --- | --- |
| **Case Study** | [case-study.md](./case-study.md) | `csXXX` | กรณีศึกษาจริงจากโปรเจกต์ที่ใช้ Rust ใน Production |
| **Deep Dive** | [deep-dive.md](./deep-dive.md) | `ddXXX` | เจาะลึก Architecture และ Design ของ Framework/Library |
| **Rust Blockchain** | [rust-blockchain.md](./rust-blockchain.md) | `rbXXX` | Rust ในโลก Blockchain, Web3, Smart Contract |
| **Rust Core** | [rust-core.md](./rust-core.md) | `rcXXX` | ฟีเจอร์ระดับแกนกลางของภาษา Rust (Compiler, Type System, Borrow Checker) |
| **Rust Crates** | [rust-crates.md](./rust-crates.md) | `rccXXX` | รีวิวและวิเคราะห์ Crate ที่น่าสนใจ |
| **Rust Games** | [rust-games.md](./rust-games.md) | `rgXXX` | การพัฒนาเกมด้วย Rust (Bevy, wgpu, ECS) |
| **Rust Hacker** | [rust-hacker.md](./rust-hacker.md) | `rhXXX` | Security, Reverse Engineering, CTF, Offensive/Defensive |
| **Rust Project** | [rust-project.md](./rust-project.md) | `rpXXX` | โปรเจกต์ Rust ที่น่าสนใจ เบื้องหลังการพัฒนา |
| **Rust Research** | [rust-research.md](./rust-research.md) | `rrXXX` | งานวิจัย, Paper, Formal Verification, PLT |
| **Rust Tools** | [rust-tools.md](./rust-tools.md) | `rtXXX` | เครื่องมือในระบบนิเวศ Rust (cargo plugins, profiling, CI/CD) |
| **Rust Update** | [rust-update.md](./rust-update.md) | `ruXXX` | ข่าวสาร, Release Notes, RFC, Roadmap ของ Rust |
| **Rust Web** | [rust-web.md](./rust-web.md) | `rwXXX` | Web Development ด้วย Rust (Frontend WASM, Backend, Full-stack) |

---

## วิธีใช้งาน

1. **ก่อนเขียนบทความใหม่** — เปิดไฟล์ Spec ของหมวดที่ต้องการเขียน
2. **อ่านหลักการและโครงร่าง** — ทำความเข้าใจลำดับเนื้อหาที่แนะนำ
3. **ใช้ Checklist** — ตรวจสอบบทความก่อนเผยแพร่ด้วย Checklist ท้ายแต่ละ Spec
4. **ตั้งชื่อไฟล์ตาม Convention** — ใช้รหัสหมวดตามที่กำหนดในตาราง

---

## หลักการร่วมที่ใช้กับทุกหมวด

### โครงสร้างพื้นฐานที่ทุกบทความต้องมี

1. **หัวข้อบทความ** (`# Title`) — ชัดเจน สื่อความหมาย ดึงดูด
2. **บทนำ** — วางบริบท อธิบายว่าทำไมเรื่องนี้ถึงสำคัญ
3. **เนื้อหาหลัก** — แบ่งเป็น sections ด้วย `##` heading และคั่นด้วย `---`
4. **บทสรุป** — สรุปประเด็นหลัก พร้อม Key Takeaway ใน Blockquote
5. **Credit & Reference** — อ้างอิงแหล่งที่มาทุกครั้ง

### การจัดรูปแบบ (Formatting)

- ใช้ `---` (Horizontal Rule) คั่นระหว่าง Section หลัก
- ใช้ **Blockquote** (`>`) สำหรับ Key Takeaways และข้อสรุปสำคัญ
- ใช้ **ตาราง** สำหรับการเปรียบเทียบหรือสรุปข้อมูลเชิงโครงสร้าง
- ใช้ **ตัวหนา** สำหรับคำสำคัญและชื่อเทคโนโลยี
- ใช้ `inline code` สำหรับ Technical Terms, ชื่อ Crate, คำสั่ง

### น้ำเสียงและภาษา

- ภาษาไทยเป็นหลัก ผสมศัพท์เทคนิคภาษาอังกฤษตามธรรมชาติ
- น้ำเสียงกึ่งทางการ เข้าถึงง่าย แต่ยังคงความเป็นมืออาชีพ
- เล่าเรื่องเชิง Narrative ไม่ใช่แค่ลิสต์ข้อมูลแห้งๆ
- วิเคราะห์และให้มุมมอง ไม่ใช่แค่แปลหรือสรุป

---

## การอัปเดต Spec

Spec เหล่านี้เป็น Living Documents — สามารถปรับปรุงได้ตลอดเวลาเมื่อมีบทเรียนใหม่จากการเขียนบทความจริง หากต้องการเสนอการเปลี่ยนแปลง ให้แก้ไขไฟล์ Spec ของหมวดนั้นโดยตรง

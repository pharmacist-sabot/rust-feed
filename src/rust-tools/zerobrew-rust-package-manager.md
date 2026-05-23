# "Zerobrew" รีดประสิทธิภาพ Package Manager ให้เร็วกว่าเดิม 5-20x ด้วย Rust

> 📅 วันที่เผยแพร่: 2026-01-26

จากวลี "Rewrite it in Rust" มักถูกมองว่าเป็นแค่ Meme หรือกระแสเห่อของใหม่ แต่สำหรับโปรเจกต์อย่าง Zerobrew (zb) การเปลี่ยนผ่านนี้ไม่ใช่แค่เรื่องของ Memory Safety แต่มันคือการรื้อถอนสถาปัตยกรรมเดิมเพื่อวางรากฐานใหม่ที่ Homebrew (ซึ่งเขียนด้วย Ruby) ทำไม่ได้ ผลลัพธ์ที่ได้คือ Cold Install ที่เร็วขึ้นถึง 5 เท่า และ Warm Reinstall ที่เร็วระดับ "Instant" หรือ 20 เท่า ความเร็วดังกล่าวไม่ได้เกิดขึ้นจากเวทมนตร์ แต่เกิดจากการออกแบบที่ประสาน 3 แกนหลักเข้าด้วยกันอย่างลงตัว 1. Architecture Discipline 2. Hybrid Concurrency Model และ 3. Low-level Syscall Integration

การออกแบบ Architecture ที่แยกความรับผิดชอบอย่างเด็ดขาด (Separation of Concerns) ภายใน Workspace ของ Zerobrew เราจะเห็นการแบ่ง `zb_core` และ `zb_io` ออกจากกันอย่างชัดเจน โดย `zb_core` ทำหน้าที่เป็น "Pure Logic" ที่ปราศจาก Side Effects ทั้งปวง รับผิดชอบงานที่ซับซ้อนอย่าง Dependency Resolution และ Graph traversal

สิ่งที่น่าสนใจคือการเลือกใช้ Data Structure อย่าง `BTreeMap` และ `BTreeSet` แทน HashMap ทั่วไป เพื่อสร้าง Deterministic behavior รับประกันว่าทุกครั้งที่ Resolve dependency graph ลำดับของ Memory layout และ Execution plan จะเหมือนเดิมเสมอ ทำให้ง่ายต่อการ Test และคาดเดาผลลัพธ์ ก่อนที่จะส่งไม้ต่อให้ `zb_io` ซึ่งเป็นเลเยอร์ที่ต้องเปื้อนฝุ่นกับ Network และ Disk

เมื่อเข้าสู่เลเยอร์ของ I/O ความท้าทายของ Package Manager คือการจัดการกับ Workload ที่ผสมผสานกัน ระหว่างงานที่รอ Network (I/O Bound) และงานที่ต้องแก้ Binary (CPU Bound) Zerobrew แก้ปัญหานี้ด้วย Hybrid Concurrency Model เริ่มต้นที่ฝั่ง Network ทีมพัฒนาเลือกใช้ `reqwest` บน `tokio` runtime แต่ไม่ได้หยุดแค่การยิง request แบบ async ทั่วไป

ใน `zb_io/src/download.rs` มีการ implement เทคนิค Racing Connections คือการเปิด Connection ขนานกันไปยัง CDN หลายจุดพร้อมกัน ใครตอบกลับมาเป็น byte แรกก่อนคือผู้ชนะ (First-response wins) เพื่อลด Latency จาก CDN Edge ที่อาจจะช้าในบางจังหวะ ผสานกับการจูน Connection Pooling ให้ reuse connection อย่างดุดัน ทำให้ Overhead ในการทำ Handshake ลดลงอย่างมีนัยสำคัญ

แต่ความเร็วในการดาวน์โหลดจะสูญเปล่าถ้าคอขวดไปอยู่ที่การเขียนลง Disk Zerobrew จึงใช้โมเดล Streaming Pipeline ผ่าน Channel (`mpsc`) ทันทีที่ Download stream แรกเสร็จสิ้น process การ Extract จะเริ่มทำงานทันทีโดยไม่ต้องรอให้คิวทั้งหมดจบ และเมื่อถึงจุดที่ต้องใช้ CPU หนักๆ อย่างการ Patch `@@HOMEBREW_PREFIX@@` ในไฟล์ Mach-O หรือการทำ Code Signing ซึ่งเป็นงานที่ Block event loop ได้ง่ายๆ Zerobrew ตัดสินใจเปลี่ยนบริบทการทำงานจาก Tokio ไปใช้ Rayon เพื่อทำ Parallel Iteration กระจายงานเข้าทุก Core ของ CPU อย่างเต็มเม็ดเต็มหน่วย การสลับ Runtime ที่เหมาะสมกับประเภทงานเช่นนี้ คือกุญแจสำคัญที่ทำให้ Resource ของเครื่องถูกรีดออกมาใช้จนหยดสุดท้าย

ความเร็วระดับ Millisecond ในการ Reinstall เกิดขึ้นได้เพราะ Content-Addressable Store (CAS) ที่ออกแบบมาคล้ายกับ Git หรือ Nix ภายใต้ `zb_io/src/store.rs` ไฟล์ทุกไฟล์จะถูกเก็บโดยอิงตาม SHA256 checksum ทำให้เกิดคุณสมบัติ Immutability และ Deduplication โดยธรรมชาติ

หากเราต้องการติดตั้ง Package เวอร์ชันต่างกันแต่มี Binary ภายในเหมือนกัน Zerobrew จะรู้ทันทีและไม่เสียเวลาดาวน์โหลดซ้ำ ความถูกต้องของข้อมูล (Correctness) ถูกค้ำประกันด้วย Atomic Operations การเขียนไฟล์จะทำใน Temp directory ให้เสร็จสมบูรณ์ก่อน แล้วจึงใช้ `rename` syscall เพื่อย้ายไปยังตำแหน่งจริง การันตีว่า Store จะไม่มีวันอยู่ในสถานะ Corrupted state แม้ไฟจะดับระหว่างติดตั้ง

และจุดที่เป็น Secret Sauce ของโปรเจกต์นี้จริงๆ คือขั้นตอน Materialization หรือการนำไฟล์จาก Store มาสร้างเป็นโครงสร้างให้ User ใช้งาน แทนที่จะใช้การ Copy ไฟล์แบบดั้งเดิมที่ช้าและเปลืองพื้นที่ Zerobrew เจาะลึกไปถึงระดับ OS ด้วยการเรียกใช้ APFS Clonefile ผ่าน FFI (`unsafe { clonefile(...) }`) ใน macOS ซึ่งเป็นเทคนิค Copy-on-Write (CoW) ระดับ Filesystem ผลลัพธ์คือการจำลองไฟล์ขนาดหลาย Gigabyte ได้ในเวลาเกือบเป็นศูนย์และไม่กินพื้นที่ Disk เพิ่มเติม หาก OS ไม่รองรับ ก็ยังมี Fallback strategy ที่ฉลาด โดยจะถอยไปใช้ Hardlink หรือ Copy ธรรมดาตามลำดับ

บทสรุปของ Zerobrew จึงเป็นมากกว่าแค่การเขียนโปรแกรมด้วย Rust แต่มันคือ Case Study ชั้นดีที่แสดงให้เห็นว่า "Fearless Concurrency" และ "Zero-cost Abstractions" เมื่อถูกนำมาใช้โดยเข้าใจธรรมชาติของ System Programming ตั้งแต่การจัดการ Memory Layout ใน Core Logic การบริหาร Thread Pool ไปจนถึงการคุยกับ Kernel ผ่าน Syscall สามารถปลดล็อกประสิทธิภาพที่ภาษา High-level ยุคเก่าไม่อาจเอื้อมถึงได้อย่างไร

อย่างไรก็ตาม Software Engineering คือศิลปะของการจัดการ Trade-off และ Zerobrew ในเวอร์ชันปัจจุบันยังคงนิยามตัวเองว่าเป็น Experimental Software ที่ซื่อสัตย์ต่อข้อจำกัด แม้สถาปัตยกรรมพื้นฐานจะพิสูจน์แล้วว่าทำงานได้อย่างรวดเร็วกับ Core Packages ส่วนใหญ่ของ Homebrew แต่จักรวาลของ Formula นั้นกว้างใหญ่และเต็มไปด้วย Edge Cases เฉพาะตัว (เช่น Custom Build Scripts หรือความซับซ้อนของ Dependency Graph ในบาง Packages) ที่ยังต้องอาศัยการจูนเพิ่มเติม

นี่จึงเป็นพื้นที่เปิดกว้างสำหรับ Rust Community ที่จะเข้ามามีส่วนร่วม ไม่ใช่แค่ในฐานะผู้ใช้งานที่ต้องการความเร็ว แต่ในฐานะ Contributor ที่จะช่วยกัน Submit Issues หรือ PRs เพื่อถมช่องว่างเหล่านั้น เปลี่ยน Proof of Concept ที่สถาปัตยกรรมแข็งแกร่งนี้ ให้กลายเป็น Production-grade Tool ที่สมบูรณ์แบบและครอบคลุมทุกการใช้งานในอนาคตครับ

**Credit & Reference:**

1. [GitHub Zerobrew](https://github.com/lucasgelfond/zerobrew)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->

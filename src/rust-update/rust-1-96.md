# Rust 1.96.0 มาแล้ว

> 📅 วันที่เผยแพร่: 2026-05-29

มาเมื่อวานนี่เองครับสำหรับ Rust 1.96.0 นึกว่าจะมาไม่ทันเดือนพฤษภาคมซะแล้ว มาดูกันครับว่ามีอะไรน่าสนใจบ้าง ไฮไลท์สำคัญคือการเข้าไปแก้ Pain point คลาสสิกอย่างเรื่อง Range หลายคนคงเคยขัดใจที่ type เก่าใน `core::ops` ไม่สามารถเป็น `Copy` ได้ สาเหตุเพราะ design เดิมมันดันไป implement `Iterator` ตรง ๆ ซึ่งถ้ายอมให้เป็น `Copy` ด้วยจะเกิด Footgun ทันที (State ของ Iterator จะทำงานซ้อนทับกันจนพัง) ในเวอร์ชันนี้ RFC3550 จึงได้ถูก Stabilize โดยสร้าง type ชุดใหม่แยกไปที่ `core::range` และเปลี่ยนให้ implement แค่ `IntoIterator` แทน ทำให้ตอนนี้ Range type สามารถเป็น `Copy` ได้อย่างสมบูรณ์แบบโดยไม่ต้องกังวลเรื่อง State อีกต่อไป

การปรับเปลี่ยนนี้ปลดล็อก Pattern การเขียนโค้ดที่คล่องตัวขึ้นมาก ตัวอย่างที่เห็นชัดคือการสร้าง Struct สำหรับ Slice accessor เช่น `struct Span(Range<usize>)` ตอนนี้เราสามารถใส่ `#[derive(Clone, Copy)]` ให้มันได้เลยโดยไม่ต้องแยกเก็บ `start` และ `end` ให้วุ่นวาย นอกจากนี้ `RangeInclusive` ตัวใหม่ยังเปิด Field ให้เป็น Public แล้ว เพราะไม่จำเป็นต้องซ่อนสถานะของ Iterator อีกต่อไป สำหรับคนที่เขียน Library แนะนำให้ปรับ Public API มารับค่าเป็น `impl RangeBounds` แทน เพื่อให้ฟังก์ชันของเรารองรับทั้ง Range แบบ Legacy และแบบใหม่ได้อย่างไร้รอยต่อ

ในมุมของ Developer Experience (DX) เวอร์ชันนี้ได้ Stabilize macro อย่าง `assert_matches!` และ `debug_assert_matches!` แล้ว จากเดิมที่เรามักจะใช้ `assert!(matches!(...))` ซึ่งเวลาเทสพังมันจะไม่บอกรายละเอียดอะไรเลย ตัว Macro ใหม่นี้จะ Panic พร้อมกับพ่นค่าผ่าน Debug representation ออกมาให้เราวิเคราะห์หาสาเหตุได้ง่ายขึ้น อย่างไรก็ตามเพื่อป้องกันชื่อชนกับ Crate ยอดฮิตภายนอก ตัว Macro นี้จะไม่ถูกรวมอยู่ใน Standard Prelude เราจึงต้อง Import เองจาก `core` หรือ `std` เสมอ

มาดูฝั่ง Target และ Security กันบ้าง สำหรับคนที่คอมไพล์ลง WebAssembly ตอนนี้ Linker จะเข้มงวดขึ้นและไม่อนุญาตให้ผ่าน Flag `--allow-undefined` อีกต่อไป 👏 ถ้ามี Undefined symbol โผล่มาตอน Link จะเกิด Error ทันที แทนที่จะแอบแปลงเป็น Import จากโมดูล "env" เหมือนแต่ก่อน ซึ่งช่วยดักบั๊กเรื่องการตั้งชื่อ Symbol ผิดตั้งแต่ตอนบิลด์ ปิดท้ายด้วยเรื่องความปลอดภัย Cargo มีการแพตช์ช่องโหว่ CVE ระดับ Medium และ Low สองจุดที่เกี่ยวกับกระบวนการแตกไฟล์ Symlink ใน Tarball และการทำ URL Auth (ซึ่งคนใช้งานผ่าน crates.io ตามปกติจะไม่ได้รับผลกระทบใด ๆ)

นอกจากนี้ยังมี API ดี ๆ อย่าง `LazyCell` และ `LazyLock` ที่เพิ่ง Stabilize ด้วย ใครสนใจลองสั่ง `rustup update stable` ได้เลยครับ

**Credit & Reference:**

1. [Announcing Rust 1.96.0](https://blog.rust-lang.org/2026/05/28/Rust-1.96.0/)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->

# พาทำเกม 2D RPG ด้วย Bevy ฉบับ "โปรแกรมเมอร์ใจร้อน"

> 📅 วันที่เผยแพร่: 2026-01-24

สำหรับใครที่อยากลองเขียนเกมด้วย Rust (เขาว่า...เรียนรู้ภาษาโปรแกรมมิ่งผ่านการเขียนเกมจะทำให้เข้าใจภาษาได้มากขึ้น จริงหรือ??) ผมไปเจอโปรเจกต์นี้มา น่าสนใจมากครับ ล่าสุดอัพเดตถึงบทที่ 6 ละครับ

ความเจ๋งคือเขาไม่ได้โยนโค้ดก้อนใหญ่มาตูมเดียวให้งงครับ แต่จะพาเราไต่ระดับไปทีละ Chapter เริ่มตั้งแต่บทแรกที่แค่เสกตัวละครให้เดินได้ แล้วค่อยๆ เติมระบบ Procedural Generation สร้างแผนที่สุ่มด้วย WFC ในบทที่สอง พอถึงบทหลังๆ เกมเราจะมีทั้งระบบ Inventory, Collision ที่ซับซ้อนขึ้น ไปจนถึงการเขียน Custom Shader ทำเอฟเฟกต์ Particle แสงวิบวับสวยงาม

ใครที่ชอบเรียนรู้ผ่านการแกะโค้ดน่าจะถูกใจ เพราะโครงสร้างโปรเจกต์เขาแยก Folder ตามบทไว้ชัดเจน (Chapter 1-6 ล่าสุด) ทำให้เราเห็นพัฒนาการของโค้ดและการออกแบบระบบ Plugin ต่างๆ ใน Bevy ได้ง่ายมาก เรียกว่าโหลดมาลอง `cargo run` ดูทีละอันก็เข้าใจภาพรวมการทำเกมด้วย Rust ได้เยอะเลยครับ

ลองเข้าไปดู Source Code หรือเอามาลองรันเล่นกันได้ที่ลิงก์ด้านล่างนี้ครับ ถือเป็น Resource ที่คุณภาพดีมากสำหรับสายเกม โปรเจกต์นี้เป็น Source code ประกอบบทเรียนของคุณ James Febin (Impatient Programmer) นะครับ ยกเครดิตให้เลยว่าทำออกมาได้ละเอียดยิบๆๆเลยครับ

**Credit & Reference:**

1. [GitHub repo](https://github.com/jamesfebin/ImpatientProgrammerBevyRust)
2. [Blog post](https://aibodh.com/)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->

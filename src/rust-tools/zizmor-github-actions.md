# มีใครเคยใช้ zizmor กันแล้วบ้าง

> 📅 วันที่เผยแพร่: 2026-06-10

จาก awesome-rust เครื่องมือในหมวดหมู่ Static analysis มีไม่ค่อยเยอะ ล่าสุดมีตัวหนึ่งที่เตะตาผมคือเจ้า `zizmor` นี่แหละครับ มันโฟกัสไปที่ GitHub Actions โดยตรงเลย แน่นอนว่าหากใครเป็นนักพัฒนาบน GitHub น่าจะต้องมี GitHub Actions อย่างน้อยก็สักไฟล์หนึ่งน่าา ส่วนของผมนั้นใช้เกือบทุกโปรเจกต์ และพอ scan ก็เจอเรื่อง `persist-credentials: false` เลยขอพูดก่อนนิดหนึ่งครับ

อธิบายเรื่อง `persist-credentials: false` เพิ่มนิดนึงครับ ตอน `actions/checkout` รัน มันจะ clone ลง runner แล้วเขียน git config ฝัง token ของ workflows (`GITHUB_TOKEN`) ไว้ใน `.git/config` เพื่อให้ step ถัด ๆ ไป push กลับได้สะดวก ปัญหาคือ token นั้นจะอยู่ในไฟล์บนดิสก์ตลอดทั้ง job ใคร (Hacker) อ่าน `.git/config` ก็เห็น ถ้า step ถัดไปรันโค้ดที่ไม่น่าไว้ใจ (เช่น `npm install` แพ็กเกจที่ถูก compromise) โค้ดนั้นสามารถ `cat .git/config` แล้วขโมย token ไปใช้ได้เลย

ซึ่ง `persist-credentials: false` นั้น มันจะสั่งให้ checkout ลบ token ออกจาก `.git/config` ทันทีหลัง clone เสร็จ ซึ่งคำสั่งนี้เราเพิ่มแค่บรรทัดเดียวเอง แต่ประโยชน์นั้นผมว่าหลายคนมองข้ามไป 555 (รวมถึงผมด้วย)

การติดตั้งใช้งานไม่ยากครับ มีหลายวิธีสามารถไปอ่านใน doc ได้ แต่ผมใช้ cargo ก็สะดวกดีครับ `cargo install --locked zizmor` รอแป๊บหนึ่งก็ติดตั้งเสร็จละครับ เช็คว่าติดตั้งสำเร็จด้วย `zizmor --version` ทีนี้ก็ scan กันสนุกสนานละครับ ผมใช้ Windows ก็ใช้คำสั่งว่า `zizmor ./.github/workflows/` ตรวจมันทุกไฟล์ yml หลัก ๆ ที่เจอก็จะมีเรื่อง persist-credentails: false มีเรื่องของ permission บ้าง ที่ไม่ได้ระบุให้ชัดเจน แต่ที่เจอเยอะก็นี่เลย commit SHA ปกติเราก็จะเอาง่าย ๆ ว่า v... ก็พอละ และยิ่ง AI ก็จะมาแบบ `actions/checkout@v4` ท่ามาตรฐานเลย แม้ปัจจุบันจะไปไกลถึง v6 แล้วก็ตาม

ยุคนี้การเจาะระบบเจอถี่เหลือเกิน อะไรที่เราพอทำได้ผมว่าทำไว้ดีกว่าไม่ทำนะครับ วันนี้ก็เพิ่งอ่านบทความของเพจ ISAN FANPAGE ที่ว่าโครงการ Open Source ของ Microsoft บน GitHub โดน Malware Injection เพื่อขโมย Credentials แล้วชอบตอนท้ายที่ว่า "อย่าเชื่อเพียงว่า เป็น Official Repo มีดาวเยอะ เป็น Microsoft แล้วจะ OK แน่ ๆ" ซึ่งแนวคิด Zero Trust จึงสำคัญมาก ๆ ๆ ๆ ๆ ว่าแล้วก็อย่าลืม `zizmor` นะครับ

**Credit & Reference:**

1. [GitHub zizmor](https://github.com/zizmorcore/zizmor)
2. [Docs zizmor](https://docs.zizmor.sh)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->

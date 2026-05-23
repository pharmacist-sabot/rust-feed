# เมื่อ Rust กลายเป็นกุญแจสำคัญในการไขปริศนา Cryptography ยุคหลังควอนตัม

> 📅 วันที่เผยแพร่: 2026-01-25

ในการพัฒนา Systems Programming และ Cryptography เรามักได้ยินเสมอว่า Rust คือภาษาแห่งอนาคตด้วยเรื่องของ Memory Safety แต่เมื่อเร็วๆ นี้ มีงานวิจัยชิ้นหนึ่งชื่อว่า "Benchmarking of the Amortized Post Quantum Combiner for MLS" ซึ่งผมมองว่าเป็น Showcase ที่แสดงศักยภาพของ Rust ได้อย่างยอดเยี่ยม งานนี้ไม่ใช่แค่การนำเสนอทฤษฎีทางคณิตศาสตร์ แต่เป็นการลงมือ Implement ระบบ Messaging Layer Security (MLS) ที่ต้องรองรับมาตรฐานความปลอดภัยยุคหลังควอนตัม (Post-Quantum Cryptography - PQC) โดยใช้ Rust ecosystem ขับเคลื่อนทั้งระบบ เพื่อพิสูจน์ว่าเราสามารถสร้างระบบที่มีความปลอดภัยสูงสุดได้โดยไม่สูญเสีย Performance ไปกับ Overhead มหาศาลของอัลกอริทึมยุคใหม่

เรื่องราวเริ่มต้นที่ความท้าทายของโปรโตคอล MLS (RFC 9420) ซึ่งเป็นมาตรฐานใหม่ของการแชทแบบกลุ่มที่ต้องเผชิญกับภัยคุกคามจากคอมพิวเตอร์ควอนตัม การจะนำมาตรฐาน NIST PQC อย่าง ML-KEM หรือ ML-DSA เข้ามาใช้นั้นแลกมาด้วยต้นทุนที่สูงลิบลิ่ว ทั้งขนาดของ Key, Ciphertext และ Signature ที่ใหญ่กว่าเดิมหลายเท่าตัว

ทีมวิจัยจาก Naval Postgraduate School จึงนำเสนอแนวคิด "Amortized Post Quantum (APQ) Combiner" ซึ่งเป็นการรันสอง Session ขนานกัน คือ Session แบบ PQC ที่หนักหน่วง และ Session แบบดั้งเดิมที่รวดเร็ว โดยใช้เทคนิคการ "เฉลี่ยต้นทุน" (Amortization) เพื่อลดภาระการประมวลผล แต่ความน่าสนใจสำหรับพวกเราชาว Rustacean อยู่ที่วิธีการที่พวกเขาแปรเปลี่ยนแนวคิดซับซ้อนนี้ให้เป็น Code ที่ทำงานได้จริง

ทีมวิจัยเลือกใช้ `OpenMLS` ซึ่งเป็น Implementation ของ MLS ในภาษา Rust เป็นฐานในการพัฒนา โดยสิ่งที่น่าสนใจคือการใช้ประโยชน์จาก Architecture ของ Rust ที่มีความ Modular สูง ผ่านระบบ Traits พวกเขาไม่ได้ Hard-code อัลกอริทึมลงไปตรงๆ แต่ใช้ Pattern ของ Modular Crypto-providers เพื่อ Inject backend ที่ต้องการเข้าไป ในที่นี้พวกเขาเลือกใช้ `OpenMLS-RustCrypto` แทนที่จะเป็น Libcrux เพื่อความเข้ากันได้ของ CPU ที่กว้างกว่า โดยมีการดึง Crates จากตระกูล `RustCrypto` อย่าง `mldsa` และ `mlkem` เข้ามาใช้งานร่วมกับ `hpke` crate (Hybrid Public Key Encryption)

ซึ่งตรงนี้เองที่มีความท้าทายเชิงวิศวกรรมซ่อนอยู่ เพราะ `RustCrypto ML-KEM` นั้นเป็น Primitive ดิบๆ ทีมพัฒนาจึงต้องเขียน Custom Handler หรือ Glue Code ขึ้นมาเพื่อ Wrap ให้ `ml-kem` สามารถคุยรู้เรื่องกับ Generic Interface ของ `hpke` ได้อย่าง Type-safe ซึ่งสะท้อนความยืดหยุ่นของ Type System ใน Rust ที่ช่วยจัดการความซับซ้อนระดับ Low-level ได้อย่างหมดจด

ในส่วนของ Logic การทำงานแบบ APQ นั้นมีความซับซ้อนในการจัดการ State สูงมาก ระบบต้องรัน State Machine สองตัวไปพร้อมกัน โดย Session หลักที่เป็น PQC จะทำงานนานๆ ครั้ง (เรียกว่า Full Commit) เพื่อ Derive เอา Entropy หรือความลับออกมา แล้วแปลงเป็น Pre-Shared Key (PSK) เพื่อ Inject ข้ามไปยัง Session แบบดั้งเดิม (Partial Commit) การจัดการ Flow ข้อมูลข้าม Session โดยที่ต้องรักษา Forward Secrecy และ Post-Compromise Security ไว้ด้วยนั้น หากเป็นภาษาอื่นอาจเสี่ยงต่อ Memory Corruption หรือ Logic Error ได้ง่าย แต่ด้วย Ownership Model ของ Rust ทำให้การจัดการ `export_secret` และการส่งต่อ PSK ผ่าน Proposal mechanism ของ MLS เป็นไปได้อย่างรัดกุมและตรวจสอบได้ตั้งแต่ตอน Compile

เพื่อให้มั่นใจในประสิทธิภาพ ทีมวิจัยใช้ Tooling มาตรฐานของ Rust Community อย่างเข้มข้นในการวัดผล พวกเขาใช้ `Criterion` ในการทำ Micro-benchmarking เพื่อวัด Execution Time โดยรันซ้ำและตัด Noise รบกวนออก และใช้ `Hotpath` ในการทำ Profiling การใช้หน่วยความจำแบบเจาะลึก (Granular Peak RAM Usage) เพื่อดูพฤติกรรมของฟังก์ชันอย่าง `encrypt` หรือ `sign` โดยเฉพาะ ผลลัพธ์ที่ได้นั้นน่าทึ่งมาก เพราะมันพิสูจน์ว่าจุดที่คุ้มค่าที่สุด (Sweet Spot) อยู่ที่อัตราส่วน 1:50 (ทำ PQC 1 ครั้ง ต่อแบบดั้งเดิม 50 ครั้ง) ซึ่งที่จุดนี้ Rust สามารถรีดประสิทธิภาพจนทำให้ Message Size และ Memory Usage ลดลงแบบ Exponential เมื่อเทียบกับการรัน PQC แบบเพียวๆ หรือเทียบกับ Hybrid KEM ปกติอย่าง X-Wing

บทสรุปของงานวิจัยนี้ยืนยันว่า Rust มีความพร้อมอย่างยิ่งสำหรับงาน Cryptography ยุคใหม่ ไม่ใช่แค่เพราะความปลอดภัยของหน่วยความจำ แต่เพราะ Ecosystem ที่แข็งแกร่งอย่าง `RustCrypto` และความสามารถในการเขียน Abstraction ผ่าน Traits ที่ทำให้เราสามารถจัดการกับความซับซ้อนของอัลกอริทึม PQC (รวมถึงโหมด Authenticity ที่กินทรัพยากรสูง) ให้ทำงานบนอุปกรณ์ที่มีทรัพยากรจำกัดได้จริง โดยมี Peak Memory Usage อยู่ในระดับเพียง 1.2MB - 1.8MB เท่านั้น นี่คือหลักฐานเชิงประจักษ์ว่า Rust คือเครื่องมือที่ทรงพลังที่สุดในมือนักพัฒนาที่จะสร้างระบบความปลอดภัยสำหรับโลกยุคหน้าครับ

**Credit & Reference:**

1. [OpenMLS (The Core Protocol)](https://github.com/openmls/openmls)
2. [Rust Crypto](https://github.com/RustCrypto)
3. [ML-KEM (Kyber)](https://github.com/RustCrypto/KEMs)
4. [ML-DSA (Dilithium)](https://github.com/RustCrypto/signatures)
5. [HPKE (Hybrid Public Key Encryption)](https://crates.io/crates/hpke)
6. [Criterion.rs เครื่องมือทำ Micro-benchmarking](https://github.com/bheisler/criterion.rs)
7. [Hotpath-rs เครื่องมือทำ Process Profiling](https://github.com/pawurb/hotpath-rs)
8. [Source Code ของงานวิจัย](https://github.com/lwerrors/SSR2025-Tian-APQ-MLS)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->

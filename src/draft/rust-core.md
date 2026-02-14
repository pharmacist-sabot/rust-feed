# Rust Core — โครงร่างมาตรฐานบทความ

> **รหัสหมวด:** RC
> **รูปแบบชื่อไฟล์:** `rcXXX-name.md` (เช่น `rc001-borrow-checker-internals.md`)
> **รูปแบบชื่อไฟล์ใน SUMMARY.md:** [RC001: เจาะลึก Borrow Checker](./rc001-borrow-checker-internals.md)
> **จุดประสงค์ของหมวดนี้:** อธิบายฟีเจอร์ระดับแกนกลางของภาษา Rust เช่น Compiler Internals, Type System, Borrow Checker, Memory Model, Trait System, Lifetime, Unsafe Rust และกลไกพื้นฐานที่ทำให้ Rust เป็น Rust

---

## หลักการสำคัญของหมวด Rust Core

- ต้องเกี่ยวข้องกับ **ตัวภาษา Rust โดยตรง** (ไม่ใช่ Crate หรือ Framework ภายนอก)
- ครอบคลุมเรื่อง Compiler, Type System, Trait System, Lifetime, Ownership, Borrowing, Unsafe, MIR/HIR, Codegen, Macro System, Const Generics, GATs ฯลฯ
- อาจเป็นฟีเจอร์ที่ Stabilize แล้ว หรือ Nightly Feature ที่กำลังจะมาก็ได้ (ระบุสถานะให้ชัดเจน)
- เน้น **"ทำไม" และ "อย่างไร"** มากกว่า **"อะไร"** — ไม่ใช่แค่บอกว่ามีฟีเจอร์นี้ แต่ต้องอธิบายว่ามันทำงานอย่างไรในระดับลึก

---

## โครงร่างลำดับเนื้อหา

### 1. หัวข้อบทความ (Title)

```
# [ชื่อหัวข้อที่สื่อความหมายชัดเจน]
```

- ควรมีทั้งชื่อฟีเจอร์/แนวคิดภาษาอังกฤษ และคำอธิบายสั้นๆ ภาษาไทย
- ตัวอย่าง: `# เจาะลึก GATs (Generic Associated Types): เมื่อ Rust Type System ก้าวข้ามขีดจำกัด`

---

### 2. บทนำ — ทำไมเรื่องนี้ถึงสำคัญ (1–2 ย่อหน้า)

- วางบริบทให้ผู้อ่านเข้าใจว่า **ปัญหาเดิมคืออะไร** ก่อนที่จะมีฟีเจอร์/กลไกนี้
- ชี้ให้เห็นว่าฟีเจอร์นี้ **แก้ปัญหาอะไร** หรือ **ปลดล็อกอะไร** ในเชิง Language Design
- อาจเปรียบเทียบกับภาษาอื่นสั้นๆ เพื่อให้เห็นภาพ

---

### 3. แนวคิดพื้นฐาน — Mental Model (1–2 sections)

- อธิบาย **แนวคิดเบื้องหลัง** ของกลไกนี้ในเชิง Conceptual
- ใช้ Analogy หรือ Diagram ช่วยอธิบายถ้าเป็นเรื่องซับซ้อน
- ใช้ Blockquote (`>`) เน้นหลักการสำคัญ

> **ตัวอย่าง Blockquote**
>
> อธิบายหลักการสำคัญที่ผู้อ่านต้องจำไว้

---

### 4. เจาะลึกกลไกภายใน — How It Works Under the Hood (2–3 sections)

- แต่ละ section ควรมี `---` คั่น
- อธิบาย **กลไกภายใน** ของ Compiler หรือ Runtime ที่เกี่ยวข้อง
- ยกตัวอย่าง Code ที่แสดง **Before vs After** หรือ **ทำได้ vs ทำไม่ได้**
- ถ้าเกี่ยวกับ Compiler ให้อธิบาย Phase ที่เกี่ยวข้อง (Parsing → HIR → MIR → Codegen)
- ใช้ Blockquote สำหรับ Key Insight แต่ละจุด

#### แนวทางการยกตัวอย่าง Code

- Code ต้อง **Compile ได้จริง** (หรือระบุชัดเจนว่าเป็น Pseudo-code / ยังไม่ Stable)
- แสดง **Error Message** ของ Compiler ถ้าเป็นเรื่องที่เกี่ยวกับ Compile-time Check
- เปรียบเทียบ Code ที่ **ถูกต้อง vs ผิดพลาด** เพื่อให้เห็นภาพ

---

### 5. ผลกระทบต่อ Ecosystem (1 section)

- ฟีเจอร์นี้ส่งผลต่อ **Crates ยอดนิยม** อย่างไร
- มี Crate ไหนที่ได้ประโยชน์โดยตรง หรือต้องปรับตัว
- ส่งผลต่อ **Design Patterns** ที่ชาว Rustacean ใช้กันอย่างไร

---

### 6. ข้อควรระวังและ Trade-offs (1 section)

- ฟีเจอร์นี้มี **ข้อจำกัด** หรือ **Edge Cases** อะไรบ้าง
- มี **Compile-time Cost** หรือ **Complexity** ที่เพิ่มขึ้นไหม
- สถานะปัจจุบัน: Stable / Nightly / RFC / ยังเป็นแค่ Idea

---

### 7. บทสรุป

- สรุปว่าฟีเจอร์/กลไกนี้ **เปลี่ยนแปลง** วิธีเขียน Rust อย่างไร
- มองไปข้างหน้าว่า **อนาคต** ของฟีเจอร์นี้จะเป็นอย่างไร (ถ้ามี Roadmap)
- ใช้ Blockquote สรุปข้อคิดสำคัญ

> **Key Takeaway**
>
> สรุปสิ่งสำคัญที่สุดที่ผู้อ่านควรจดจำ

---

### 8. Credit & Reference

``` text
**Credit & Reference:**
1. [ชื่อแหล่งอ้างอิงหลัก](URL)
2. [GitHub repo](URL)
3. [แหล่งอ้างอิงเพิ่มเติม](URL)
```

- **ต้องมีอย่างน้อย 1 แหล่ง** ที่เป็น Primary Source
- แนะนำให้มี Link ไปยัง Source Code / GitHub repo
- เรียงลำดับจากสำคัญที่สุดไปน้อยที่สุด

---

## Checklist ก่อนเผยแพร่

- [ ] ชื่อไฟล์ตรงตามรูปแบบ `rcXXX-topic-name.md`
- [ ] มีบทนำที่วางบริบทชัดเจน
- [ ] มี Code ตัวอย่างที่ Compile ได้จริง (หรือระบุสถานะ)
- [ ] มี Blockquote เน้น Key Insight อย่างน้อย 3 จุด
- [ ] มี Section ผลกระทบต่อ Ecosystem
- [ ] มี Section ข้อควรระวังและ Trade-offs
- [ ] มีบทสรุปที่มองไปข้างหน้า
- [ ] มี Credit & Reference อย่างน้อย 1 แหล่งที่เป็น Official
- [ ] ใช้ `---` คั่นระหว่าง Section หลัก
- [ ] ภาษาไทยอ่านลื่น ศัพท์เทคนิคใช้ภาษาอังกฤษ

# Case Study (CS) — โครงร่างมาตรฐานบทความ

> **รหัสหมวด:** CS
> **รูปแบบชื่อไฟล์:** `csXXX-name.md` (เช่น `cs002-rewrite-python-to-rust.md`)
> **รูปแบบชื่อไฟล์ใน SUMMARY.md:** [CS002: การ Rewrite Python ด้วย Rust](./cs002-rewrite-python-to-rust.md)
> **จุดประสงค์ของหมวดนี้:** นำเสนอกรณีศึกษาจริงจากโปรเจกต์ที่ใช้ Rust ใน Production เพื่อให้ผู้อ่านเห็นภาพการตัดสินใจ การออกแบบ และผลลัพธ์ที่เกิดขึ้นจริง

---

## หลักการสำคัญของหมวด Case Study

1. **เน้นเรื่องจริง** — ต้องเป็นโปรเจกต์ที่มีอยู่จริง มี Source Code หรือ Reference ยืนยันได้
2. **Problem → Solution → Result** — โครงสร้างต้องพาผู้อ่านเดินทางจากปัญหา ไปสู่วิธีแก้ และผลลัพธ์
3. **วิเคราะห์เชิงลึก** — ไม่ใช่แค่บอกว่า "ใช้ Rust แล้วเร็วขึ้น" แต่ต้องอธิบายว่า **ทำไม** และ **อย่างไร**
4. **ถอดบทเรียน** — ทุกบทความต้องมีบทเรียนที่ผู้อ่านนำไปใช้ต่อได้

---

## โครงร่างลำดับเนื้อหา

### 1. หัวข้อบทความ (Title)

```
# [ชื่อบทความที่สื่อความหมาย — กระชับ ดึงดูด]
```

- ใช้ภาษาไทยเป็นหลัก อาจผสมศัพท์เทคนิคภาษาอังกฤษได้
- ควรบอก **What** (ทำอะไร) + **With What** (ด้วยอะไร) ในชื่อ
- ตัวอย่าง: `การสร้าง Media Converter ด้วย Rust`

---

### 2. บทนำ (Introduction) — 1-2 ย่อหน้า

- แนะนำ **บริบท** ของโปรเจกต์: ใครทำ ทำอะไร ทำไมถึงน่าสนใจ
- ระบุ **แก่นหลัก** ของ Case Study ว่าจะพาผู้อ่านไปเรียนรู้อะไร
- ทำให้ผู้อ่านรู้สึก **อยากอ่านต่อ** — ชูจุดเด่นที่น่าสนใจที่สุดขึ้นมาก่อน

---

### 3. ปัญหาเดิม (The Problem / Before) — 1-2 sections

- อธิบาย **Pain Points** ที่โปรเจกต์เจอก่อนใช้ Rust
- อาจเป็นปัญหาด้าน Performance, Memory, Reliability, DX หรือ Architecture
- ใช้ Blockquote `>` เพื่อเน้นตัวเลข/ข้อเท็จจริงที่สำคัญ
- ถ้าเป็นการ Rewrite จากภาษาอื่น ให้อธิบายว่าระบบเดิมใช้อะไร

**ตัวอย่างหัวข้อ:**
- `## The Cost of Runtime: ทำไมต้องหนีจาก Electron`
- `## ข้อจำกัดของระบบเดิม`

---

### 4. ทำไมถึงเลือก Rust (Why Rust?) — 1 section

- อธิบายเหตุผลที่เลือก Rust เหนือทางเลือกอื่น
- ชี้ให้เห็น **คุณสมบัติเฉพาะของ Rust** ที่ตอบโจทย์ปัญหา เช่น:
  - Memory Safety without GC
  - Zero-cost Abstractions
  - Fearless Concurrency
  - Type System / Error Handling
- ใช้ Blockquote `>` เพื่อสรุปจุดเด่น 3-5 ข้อ

---

### 5. สถาปัตยกรรมและการออกแบบ (Architecture & Design) — 2-3 sections

- **หัวใจของบทความ** — ส่วนนี้ต้องละเอียดที่สุด
- อธิบาย Architecture ในระดับ High-level ก่อน แล้วค่อย Drill down
- ใช้ Blockquote `>` เพื่อแยกรายละเอียดทางเทคนิคออกจากเนื้อเรื่อง
- ควรครอบคลุม:
  - **System Architecture** — ภาพรวมของ Component ต่างๆ
  - **Key Design Decisions** — ทำไมถึงเลือกออกแบบแบบนี้
  - **Rust-specific Patterns** — Pattern หรือ Idiom ของ Rust ที่ใช้

**ตัวอย่างหัวข้อ:**
- `## Architecture Design: Probe, Plan, Execute`
- `## Rust Async Runtime vs Node.js Event Loop`
- `## Concurrency Model`

---

### 6. ผลลัพธ์และตัวเลข (Results & Metrics) — 1 section

- แสดง **ผลลัพธ์ที่วัดได้** (Quantitative Results)
- ใช้ตารางเปรียบเทียบ Before/After ถ้าเป็นไปได้
- ตัวอย่างตัวเลขที่ควรนำเสนอ:
  - Performance (Latency, Throughput, Startup Time)
  - Resource Usage (Memory, CPU, Binary Size)
  - Reliability (Crash Rate, Error Rate)
  - Developer Productivity (Build Time, Test Coverage)

---

### 7. บทเรียนและข้อควรระวัง (Lessons Learned) — 1 section (ถ้ามี)

- สิ่งที่ **ทำแล้วได้ผลดี** (What Worked)
- สิ่งที่ **ควรระวัง** หรือ **ไม่ได้ผล** (What Didn't Work / Pitfalls)
- คำแนะนำสำหรับคนที่จะทำโปรเจกต์คล้ายกัน

---

### 8. บทสรุป (Conclusion) — 1-2 ย่อหน้า

- สรุปบทเรียนหลักจาก Case Study
- เชื่อมโยงกลับไปที่ **คุณค่าของ Rust** ในบริบทนี้
- ชี้แนะผู้อ่านไปยัง Resource เพิ่มเติม (Source Code, Documentation)
- ใช้ Blockquote `>` สำหรับ Key Takeaway สุดท้าย

---

### 9. Credit & Reference

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

- [ ] มีบทนำที่ดึงดูดและบอกบริบทชัดเจน
- [ ] อธิบายปัญหาเดิมอย่างเป็นรูปธรรม
- [ ] มีเหตุผลชัดเจนว่าทำไมถึงเลือก Rust
- [ ] อธิบาย Architecture / Design ในเชิงลึก
- [ ] มีผลลัพธ์ที่วัดได้ (ตัวเลข/เปรียบเทียบ)
- [ ] มีบทสรุปที่ชัดเจน
- [ ] มี Credit & Reference ครบถ้วน
- [ ] ใช้ Blockquote `>` เน้นจุดสำคัญทุก section
- [ ] ใช้ `---` คั่นระหว่าง section หลัก
- [ ] ชื่อไฟล์ตรงตามรูปแบบที่กำหนด

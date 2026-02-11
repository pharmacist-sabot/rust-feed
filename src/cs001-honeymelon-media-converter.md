# การสร้าง Media Converter ด้วย Rust

มี Case Study ที่น่าสนใจเกี่ยวกับการ Rewrite Desktop App จาก Electron มาเป็น Tauri (Rust Backend + Vue Frontend) ซึ่งเป็นตัวอย่างที่ชัดเจนมากว่าทำไม Rust ถึงเป็นภาษาที่ใช่สำหรับการทำ System Utility ที่ต้องการประสิทธิภาพสูง และมี Memory Safety

โปรเจกต์นี้ชื่อว่า **"Honeymelon"** เป็น Media Converter สำหรับ macOS (Apple Silicon) โดยเฉพาะ ซึ่งผู้พัฒนา (Jerome Thayananthajothy) ได้แชร์บทเรียนสำคัญในการทิ้ง Node.js runtime อันเทอะทะ มาสู่ความเร็วระดับ Native ของ Rust

---

## The Cost of Runtime: ทำไมต้องหนีจาก Electron

ปัญหาคลาสสิกของ Electron คือ Resource Overhead ครับ ผู้พัฒนาพบว่าเวอร์ชันแรกกินแรมมหาศาล (Unreasonable memory consumption) และ Binary มีขนาดใหญ่ (Bloated binary) การเปิด App ขึ้นมาเพื่อแปลงไฟล์ง่ายๆ แต่ต้องรัน Chromium ทั้งตัวเป็นเรื่องที่ Overkill เกินไป

```admonish success title="ผลลัพธ์หลังย้ายมา Tauri 2 (Rust Backend)"
- **Startup Time** แทบจะ Instant — Rust init FFmpeg capability detection แบบ Background thread ขณะที่ UI render
- **Memory Usage** ลดลงเหลือระดับ Native Utility ไม่ใช่ Web Browser
- **App Size** DMG มีขนาดเล็กลงอย่างมหาศาล
```

---

## Rust Async Runtime vs Node.js Event Loop

จุดที่น่าสนใจที่สุดคือการจัดการ Concurrency ครับ ใน Electron การจัดการ Child Process จำนวนมากผ่าน Node.js Event Loop มักจะมี Overhead ในการ Marshalling ข้อมูลผ่าน Single Thread แต่ในฝั่ง Rust Backend ของ Honeymelon ใช้ประโยชน์จาก Rust's Async Runtime ร่วมกับ Tauri's IPC layer ได้อย่างเต็มประสิทธิภาพ

```admonish note title="Concurrent FFmpeg Processing"
สามารถ handle Concurrent FFmpeg processes ได้โดยไม่มี Overhead เหมือน Node.js และมีการ Implement Logic ที่เรียกว่า **"Exclusive mode"** สำหรับ Codec ที่กินทรัพยากรสูง (เช่น AV1 หรือ ProRes):
- ระบบจะ **Lock** ไม่ให้จ็อบหนักๆ รันซ้อนกันเพื่อป้องกัน Resource Contention
- ในขณะที่จ็อบเบาๆ สามารถ **รันขนานกัน**ได้
```

---

## สิ่งที่ Rust มอบให้

การเลือก Rust ไม่ใช่แค่เรื่องความเร็ว แต่คือ **Reliability**

```admonish tip title="3 สิ่งที่ Rust มอบให้เหนือกว่า Node.js"
1. **No Null Pointer Panics** — บอกลา `undefined is not a function` หรือ Crash กลางอากาศขณะแปลงไฟล์
2. **Typed Result Values** — การใช้ `Result<T, E>` ทำให้ Error Propagation ถูกจัดการอย่างถูกต้องและครอบคลุมทุกเคส
3. **No GC Pauses** — สำคัญมากสำหรับงาน Real-time progress tracking เพราะ Rust ไม่มี Garbage Collector มาคอย interrupt ทำให้ Progress events ลื่นไหลและแม่นยำ
```

---

## Architecture Design: Probe, Plan, Execute

สถาปัตยกรรมของ Honeymelon แบ่งเป็น 3 Stage ที่น่าสนใจ โดยมีการแบ่งหน้าที่ระหว่าง Frontend (TS) และ Backend (Rust) อย่างชัดเจน

```admonish info title="3 Stages of Honeymelon"
**Stage 1 — Probe (Rust)**
Backend สั่ง `ffprobe` เพื่อดึง Metadata (Codec, Resolution, Color Primaries ฯลฯ)

**Stage 2 — Plan (TypeScript)**
Logic การตัดสินใจว่าจะ "Remux" (Copy stream) หรือ "Transcode" ย้ายมาทำที่ Frontend ทั้งหมด เพื่อลด Round-trip ไปที่ Rust backend ทำให้ User เห็น Plan ทันที (**Zero Latency UX**)

**Stage 3 — Execute (Rust)**
พระเอกของงาน — Rust จะ spawn `ffmpeg` เป็น Child process พร้อม Dedicated Thread สำหรับ parse `stderr` output แบบ Real-time เพื่อคำนวณ % progress, fps, speed
```

```admonish warning title="Atomic Writes เพื่อความปลอดภัย"
Output file จะถูกเขียนลง **Temporary path** ก่อน และใช้ **Atomic Rename** เมื่อ process เสร็จสมบูรณ์ ป้องกันไฟล์เสียหากโปรแกรม Crash หรือถูก Cancel กลางคัน
```

---

## FFmpeg License: Process Separation

หากใครที่เคยทำงานกับ FFmpeg จะรู้ว่าเรื่อง License (LGPL/GPL) นั้นปวดหัว การ Link library (`libavcodec`) เข้ากับ Rust code ตรงๆ อาจทำให้ Binary ของเราติดเงื่อนไข LGPL

```admonish note title="วิธีแก้ปัญหา License อย่างสะอาด"
Honeymelon ใช้ **Process Separation**:
- ❌ ไม่ใช้ Dynamic Linking หรือ Library Calls
- ✅ ใช้ Rust รัน FFmpeg เป็น **Separate Process**
- ✅ สื่อสารผ่าน Command-line args, Standard Streams และ File System เท่านั้น

วิธีนี้ทำให้ Code หลักของ Honeymelon สะอาด (Clean) และสามารถ Release ภายใต้ **GPL v3** ได้โดยไม่ขัดแย้งกับ License ของ Libraries
```

---

## บทสรุป

```admonish success title="The tools you choose determine the experience"
Honeymelon พิสูจน์ให้เห็นว่าการใช้ Rust คู่กับ Tauri ไม่ใช่แค่ Trend แต่มันคือการ **ปลดล็อกข้อจำกัดทาง Performance** ที่ Web Stack ทั่วไปทำไม่ได้ โดยเฉพาะงานที่ต้องยุ่งกับ System Process หนักๆ อย่าง Media Conversion
```

ใครสนใจลองไปแกะ Source Code ดูครับ เขียนด้วย Rust + Vue 3 ออกแบบมาสำหรับ macOS Apple Silicon โดยเฉพาะ เป็นตัวอย่างที่ดีมากสำหรับการจัดการ Command Execution และ Async ใน Rust ครับ

---

**Credit & Reference:**

1. [Introducing Honeymelon: A Case Study in Building a Better Media Converter](https://dev.to/.../introducing-honeymelon-a-case-study-in...) — by Jerome Thayananthajothy
2. [honeymelon GitHub repo](https://github.com/honeymelon-app/honeymelon)
3. [Honeymelon Website](https://honeymelon.app/)

# Rust Games (RG) — โครงร่างมาตรฐานบทความ

> **รหัสหมวด:** RG
> **รูปแบบชื่อไฟล์:** `rgXXX-name.md` (เช่น `rg001-bevy-ecs-deep-dive.md`)
> **รูปแบบชื่อไฟล์ใน SUMMARY.md:** [RG001: เจาะลึก Bevy ECS](./rg001-bevy-ecs-deep-dive.md)
> **จุดประสงค์ของหมวดนี้:** นำเสนอเนื้อหาเกี่ยวกับการพัฒนาเกมด้วย Rust ทั้ง Game Engine, Game Framework, ECS Pattern, Rendering, Physics, Audio และ Case Study ของเกมที่สร้างด้วย Rust

---

## หลักการสำคัญของหมวด Rust Games

- **Game Engines & Frameworks** — Bevy, Fyrox, Macroquad, ggez, Piston, Amethyst (legacy) ฯลฯ
- **ECS (Entity Component System)** — แนวคิด, การออกแบบ, Performance เปรียบเทียบ
- **Rendering & Graphics** — wgpu, Vulkan bindings, shader programming ใน Rust
- **Game Physics & Audio** — Rapier, kira, rodio และ libraries อื่นๆ
- **Game Dev Patterns** — State machines, event systems, asset pipelines ใน Rust
- **Case Study** — เกมที่สร้างด้วย Rust จริงๆ ทั้ง Indie และ Game Jam
- **WASM Game Development** — การ deploy เกม Rust ขึ้น Web ผ่าน WebAssembly
- **Procedural Generation** — Terrain, dungeons, world generation ด้วย Rust

---

## โครงร่างลำดับเนื้อหา

### 1. หัวข้อบทความ (Title)

```
# [ชื่อบทความที่สื่อความหมายชัดเจน]
```

- ควรระบุชื่อ Engine/Framework/เกม ที่กำลังพูดถึง
- สื่อให้เห็นว่าผู้อ่านจะได้อะไรจากบทความ
- ตัวอย่าง: _"เจาะลึก Bevy ECS — ทำไม Entity Component System ถึงเปลี่ยนวิธีคิดในการสร้างเกม"_

---

### 2. บทนำ — ทำไมต้องสร้างเกมด้วย Rust? (1–2 ย่อหน้า)

เปิดเรื่องด้วยการวาง Context ว่าทำไมหัวข้อนี้ถึงน่าสนใจสำหรับ Game Developer

- **Pain Point** ของการพัฒนาเกมแบบเดิม (C++, Unity/C#, Godot/GDScript) ที่ Rust เข้ามาแก้ได้
- **Promise** ของ Rust ในโลก Game Dev — Memory Safety without GC, Fearless Concurrency, Zero-cost Abstractions
- ระบุ **กลุ่มเป้าหมาย** ของบทความ (เช่น Beginner Game Dev, Experienced Rustacean ที่อยากลองทำเกม)

> **แนวทาง:** เน้นให้ผู้อ่านเห็นว่า Rust ไม่ใช่แค่ภาษา Systems Programming แต่เหมาะกับ Game Dev จริงๆ พร้อมเหตุผลที่ชัดเจน

---

### 3. ภาพรวมของ Engine / Framework / เกม (1–2 sections)

#### 3.1 แนะนำตัวเอกของบทความ

- **ชื่อ Engine/Framework/เกม** คืออะไร ใครเป็นผู้สร้าง
- **ปรัชญาการออกแบบ** — เช่น Bevy เน้น Modular + Data-driven, Fyrox เน้น Traditional Scene Editor
- **สถานะปัจจุบัน** — Stable? Alpha? Active Development?
- **Community & Ecosystem** — ขนาด Community, จำนวน Contributors, Plugins/Assets ที่มี

#### 3.2 เปรียบเทียบกับทางเลือกอื่น (ถ้าเหมาะสม)

> | เกณฑ์ | Engine A (Rust) | Engine B (เปรียบเทียบ) |
> | --- | --- | --- |
> | Architecture | ... | ... |
> | Performance | ... | ... |
> | Ecosystem Maturity | ... | ... |
> | Learning Curve | ... | ... |
> | Platform Support | ... | ... |

---

### 4. สถาปัตยกรรม / Technical Deep Dive (2–3 sections)

หัวใจหลักของบทความ — เจาะลึกทางเทคนิค

#### 4.1 Core Architecture

- **Design Pattern หลัก** ที่ใช้ (ECS, Scene Graph, Actor Model ฯลฯ)
- **Data Flow** — ข้อมูลไหลอย่างไรจาก Input → Logic → Rendering
- แสดง Diagram หรืออธิบาย Pipeline ให้เห็นภาพ

> **Blockquote:** สรุป Architecture ให้เข้าใจง่ายใน 3–5 บรรทัด

#### 4.2 Rust-specific Patterns ใน Game Dev

- **Ownership & Borrowing** ส่งผลต่อ Game State Management อย่างไร
- **ECS vs OOP** — ทำไม ECS ถึงเข้ากับ Rust ได้ดีกว่า Inheritance-based OOP
- **Concurrency** — Multi-threaded Systems, Parallel Queries, Async in Games
- **Unsafe Usage** — มีจุดไหนที่ต้องใช้ `unsafe` และทำไม (เช่น FFI กับ Graphics API)

#### 4.3 Performance Characteristics

- **Benchmarks** หรือ Performance Comparison (ถ้ามี)
- **Memory Layout** — Cache-friendly data structures, SoA vs AoS
- **Compile Time** — ผลกระทบจาก proc macros, generic monomorphization
- **Runtime Performance** — FPS, Frame Time, Memory Usage

> **Blockquote:** สรุปจุดแข็ง/จุดอ่อนด้าน Performance

---

### 5. ตัวอย่าง Code / Implementation Walkthrough (1–2 sections)

#### 5.1 Code Example ที่เป็นรูปธรรม

- แสดง Code Snippet ที่สำคัญพร้อมคำอธิบาย
- **อย่าแค่แปะ Code** — อธิบายว่าแต่ละส่วนทำอะไร ทำไมถึงเขียนแบบนี้
- เน้น Pattern ที่เป็น Idiomatic Rust ใน Game Dev context

#### 5.2 Game Loop / System Design

- แสดงตัวอย่างการออกแบบ Game Loop หรือ System
- อธิบาย Lifecycle: Setup → Update → Render → Cleanup
- ถ้าเป็น ECS-based ให้แสดง System, Component, Resource ตัวอย่าง

> **แนวทาง:** Code ควรสั้น กระชับ แต่สมบูรณ์พอที่จะเข้าใจ Pattern ได้ ไม่ต้องเป็น full project

---

### 6. Developer Experience & Tooling (1 section)

- **Development Workflow** — Hot Reloading, Asset Pipeline, Scene Editor
- **Debugging Tools** — Profiler, Inspector, Debug Rendering
- **Testing** — Unit test สำหรับ Game Logic, Integration test
- **Build & Distribution** — Cross-compilation, WASM builds, Platform-specific packaging
- **Pain Points** — Compile time, Learning curve, Missing features เมื่อเทียบกับ mature engines

---

### 7. Community & Ecosystem (1 section — optional)

- **Notable Games** สร้างด้วย Engine/Framework นี้
- **Plugin Ecosystem** — มี Plugin อะไรที่น่าสนใจบ้าง
- **Learning Resources** — Books, Tutorials, YouTube channels ที่แนะนำ
- **Game Jams** — ผลงานจาก Ludum Dare, Bevy Jam ฯลฯ

---

### 8. บทสรุป

- สรุปว่า **Rust เหมาะกับ Game Dev ในบริบทไหน** (และไม่เหมาะในบริบทไหน)
- **Recommendation** — ควรลองใช้ถ้า..., ยังไม่เหมาะถ้า...
- **อนาคต** — ทิศทางของ Engine/Framework นี้ หรือ Rust Game Dev โดยรวม
- ปิดด้วย **Call to Action** — ชวนให้ลองเล่น ลองสร้าง หรือเข้าร่วม Community

> **Blockquote:** ข้อความปิดที่ทรงพลัง สรุปคุณค่าหลักของบทความ

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

- [ ] หัวข้อสื่อความหมายชัดเจน มีชื่อ Engine/Framework/เกม ระบุชัด
- [ ] บทนำวาง Context ว่าทำไมหัวข้อนี้สำคัญต่อ Game Dev Community
- [ ] มี Technical Deep Dive ที่เจาะลึกเพียงพอ (ไม่ใช่แค่ Introduction)
- [ ] มี Code Example ที่เป็นรูปธรรม พร้อมคำอธิบาย
- [ ] มีการเปรียบเทียบ Performance หรือ Trade-offs ที่ชัดเจน
- [ ] กล่าวถึง Pain Points อย่างตรงไปตรงมา (ไม่ใช่แค่โปรโมท)
- [ ] มี Blockquote สรุปประเด็นสำคัญในแต่ละ section
- [ ] ใช้ `---` คั่นระหว่าง section หลักเพื่อความอ่านง่าย
- [ ] มี Credit & Reference ครบถ้วน พร้อม URL
- [ ] ไฟล์ตั้งชื่อตาม Convention: `rgXXX-ชื่อหัวข้อ.md`

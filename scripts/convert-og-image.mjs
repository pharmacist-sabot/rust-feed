import { readFileSync, writeFileSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { Resvg } from "@resvg/resvg-js";

const __dirname = dirname(fileURLToPath(import.meta.url));

const svgPath = resolve(__dirname, "..", "src", "assets", "og-image.svg");
const pngPath = resolve(__dirname, "..", "src", "assets", "og-image.png");

const svgData = readFileSync(svgPath, "utf-8");

const resvg = new Resvg(svgData, {
  fitTo: {
    mode: "width",
    value: 1200,
  },
  font: {
    fontDirs: [],
    defaultFontFamily: "Consolas",
    loadSystemFonts: true,
  },
  dpi: 96,
});

const rendered = resvg.render();
const pngBuffer = rendered.asPng();

writeFileSync(pngPath, pngBuffer);

const kb = (pngBuffer.byteLength / 1024).toFixed(1);
console.log(`✔ Converted: ${svgPath}`);
console.log(`  → Output:  ${pngPath}`);
console.log(`  → Size:    ${rendered.width}×${rendered.height} px, ${kb} KB`);

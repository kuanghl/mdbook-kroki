#### Flows.

```sh
# deps
sudo apt-get install -y poppler-utils

# code
touch tex2svg.ts
npm install typescript @types/node ts-node node-tikzjax pkg commander tslib --save-dev

# test
# PDF to PNG/JPEG/TIFF/PDF/PS/EPS/SVG
npx tsc
node dist/pdf2image.js --input dist/atomistic-gpu-batching.pdf --output dist/atomistic-gpu-batching.svg
node dist/pdf2image.js --input dist/atomistic-gpu-batching.pdf --output dist/atomistic-gpu-batching.png

# package
npm run build && npm run package

# binary
./pdf2image-linux --input dist/atomistic-gpu-batching.pdf --output dist/atomistic-gpu-batching.svg
cp pdf2image-win.exe pdftocairo && cd pdftocairo
pdf2image-win.exe --input dist/atomistic-gpu-batching.pdf --output dist/atomistic-gpu-batching.svg
```
#### Flows.

```sh
# code
touch tex2svg.ts
npm install typescript @types/node @types/tar-fs ts-node node-tikzjax pkg commander  --save-dev

# test
npx tsc
node dist/tex2svg.js --file dist/sample0.tex
node dist/tex2svg.js --string "\\begin{document} \\begin{tikzpicture} \\draw (0,0) circle (1in); \\end{tikzpicture} \\end{document}"

# package
npm run build && npm run package

# binary
./tikz2svg-linux --file dist/sample2.tex > output.svg
tikz2svg-win.exe --file dist/sample2.tex > output.svg
```
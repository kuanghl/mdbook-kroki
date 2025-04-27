import tex2svg from 'node-tikzjax';
import { readFileSync, existsSync } from 'fs';
import { Command } from 'commander';

const convertTikzToSvg = async (source: string): Promise<string> => {
  try {
    const svg = await tex2svg(source, {
        // Print log of TeX engine to console. Default: false.
        showConsole: false,
        // Additional TeX packages to load. Default: {}.
        // The following example results in `\usepackage{pgfplots}\usepackage[intlimits]{amsmath}`.
        texPackages: { pgfplots: '', amsmath: 'intlimits' },
        // Additional TikZ libraries to load. Default: ''.
        // The following example results in `\usetikzlibrary{arrows.meta,calc}`.
        tikzLibraries: 'arrows.meta,calc',
        // Additional source code to add to the preamble of input. Default: ''.
        addToPreamble: '% comment',
        // Add `<defs><style>@import url('fonts.css');</style></defs>` to SVG. Default: false.
        // This could be useful if you want to embed the SVG in a HTML file.
        embedFontCss: true,
        // URL of the font CSS file. Default: 'https://cdn.jsdelivr.net/npm/node-tikzjax@latest/css/fonts.css'.
        fontCssUrl: 'https://cdn.jsdelivr.net/npm/node-tikzjax@latest/css/fonts.css',
        // Disable SVG optimization with SVGO. Default: false.
        disableOptimize: false,
      });
    return svg;
  } catch (error) {
    throw new Error(`convert failed: ${error.message}`);
  }
};

const main = async () => {
    const program = new Command();
    program
    .name('tikz2svg')
    .description('Convert TikZ code to SVG')
    .usage('<command>[options]')
    .addHelpText('after', `
Example call:
    $ node dist/tex2svg.js --file input.tikz
    $ node dist/tex2svg.js --string "\\begin{document} \\begin{tikzpicture} \\draw (0,0) circle (1in); \\end{tikzpicture} \\end{document}"`)
    .version('1.0.0')
    .option('-f, --file <path>', 'input from file')
    .option('-s, --string <content>', 'input from string')
    .parse(process.argv)
    .configureOutput({
      writeErr: (str) => process.stderr.write(`[ERROR] ${str}`)
    });

    const options = program.opts();

    let input = '';
    if (options.file) {
        if (!existsSync(options.file)) {
            program.error(`not exist: ${options.file}`);
        }
        input = readFileSync(options.file, 'utf-8');
    } else {
        input = options.string;
    }
      
    convertTikzToSvg(input)
        .then(svg => console.log(svg))
        .catch(err => console.error(err));
      
}

if (require.main === module) {
  main();
}

// // 命令行接口
// if (require.main === module) {
//   const source = `\begin{document}
//   \begin{tikzpicture}[domain=0:4]
//     \draw[very thin,color=gray] (-0.1,-1.1) grid (3.9,3.9);
//     \draw[->] (-0.2,0) -- (4.2,0) node[right] {$x$};
//     \draw[->] (0,-1.2) -- (0,4.2) node[above] {$f(x)$};
//     \draw[color=red]    plot (\x,\x)             node[right] {$f(x) =x$};
//     \draw[color=blue]   plot (\x,{sin(\x r)})    node[right] {$f(x) = \sin x$};
//     \draw[color=orange] plot (\x,{0.05*exp(\x)}) node[right] {$f(x) = \frac{1}{20} \mathrm e^x$};
//   \end{tikzpicture}
// \end{document}`;

//   convertTikzToSvg(source)
//     .then(svg => console.log(svg))
//     .catch(err => console.error(err));
// }
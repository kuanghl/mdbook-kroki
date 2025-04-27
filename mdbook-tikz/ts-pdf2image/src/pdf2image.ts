import { input,  Options} from 'node-pdftocairo';
import path from 'path';
import fs from 'fs/promises'
import { Command } from 'commander'

/**
 * PDF转图片/矢量图核心函数
 * @param inputPath 输入PDF路径
 * @param outputPath 输出文件路径
 * @param fmt 目标格式（根据文件后缀自动推断）
 */
async function convertpdf2image(
    inputPath: string,
    outputPath: string,
    fmt: string
  ): Promise<void> {
    // 参数校验
    if (!inputPath) {
      throw new Error('Missing input .pdf file path')
    }
  
    // 检查输入文件存在性
    try {
      await fs.access(inputPath)
    } catch {
      throw new Error(`Input file not found: ${inputPath}`)
    }
    
    // 构建转换选项[7,8](@ref)
    const options: Options = {
        format: fmt as Options['format'], // 类型转换保证安全性
        // 可扩展参数（如分辨率）
        resolution: 600 
    }
      
    // 执行转换
    try {
      await input(inputPath, options).output(outputPath)
    } catch (error) {
      throw new Error(`Conversion failed: ${error instanceof Error ? error.message : error}`)
    }
}

const main = async () => {
    const program = new Command();
    program
    .name('pdf2image')
    .description('Convert pdf code to PNG/JPEG/TIFF/PDF/PS/EPS/SVG')
    .usage('<command>[options]')
    .addHelpText('after', `
Example call:
    $ node dist/pdf2image.js --input dist/atomistic-gpu-batching.pdf --output dist/atomistic-gpu-batching.svg
    $ node dist/pdf2image.js --input dist/atomistic-gpu-batching.pdf --output dist/atomistic-gpu-batching.png`)
    .version('1.0.0')
    .option('-i, --input <path>', 'input file directory')
    .option('-o, --output <path>', 'output file directory')
    .parse(process.argv)
    .configureOutput({
      writeErr: (str) => process.stderr.write(`[ERROR] ${str}`)
    });

    const opts = program.opts();

    if (!opts.output) {
        opts.output = "./output.svg"
    }

    // 从输出路径提取格式[6,8](@ref)
    var format = path.extname(opts.output).slice(1)
    if (!['png','jpeg','tiff','pdf','ps','eps','svg'].includes(format)) {
        format = "svg"    
    }   

    try {
        await convertpdf2image(
            path.resolve(opts.input),
            path.resolve(opts.output),
            format
        )
        console.log('conversion completed successfully')
    } catch (error) {
        console.error(`Error: ${error instanceof Error ? error.message : error}`)
    }

    // const options = program.opts();
    // const inputPath = path.join(__dirname, './atomistic-gpu-batching.pdf');
    // const outputPath = path.join(__dirname, './atomistic-gpu-batching.png');
    // const options: Options = { format: 'png' };
    // await input(inputPath, options).output(outputPath);
}

if (require.main === module) {
    main();
}
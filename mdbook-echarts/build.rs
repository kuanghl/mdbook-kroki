fn main() {
    cc::Build::new().file("src/echarts/pikchr/pikchr.c").compile("pikchr");
}
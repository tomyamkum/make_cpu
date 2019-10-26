# make_cpu
Nand2Tetris を Rustで実装する

src/main.rsでassemble関数を動かすと(cargo run),test.asmのプログラムがアセンブルされる(test.asm->test.hack)。  
src/main.rsでcomputer関数を動かすと,バイナリファイル(test.hack)が実行される。

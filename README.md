# Sudoku difficulty analyzer
Programa, kuri iš esamų duomenų išmoksta nustatyti paduodamos sudoku lentos sunkumo lygį.

# How it works?
Programa iš `./res/sudoku_learn.json` paima pradinius duomenis pagal kuriuos nustato `./res/sudoku_test.json` faile pateiktų Sudoku lentų sunkumo lygius.

Norint pridėti kitus duomenis tiesiog pakeisti `./res/sudoku_test.json` faile esančius duomenis.

# Run program
## 1 method. Download and run the binary executable
Requirements:
- [Visual Studio C++ Build tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (Jei yra įrašytas Visual Studio, jie jau yra)

Atsisiųsti failus iš [github.com/Ertibus/sudoku-diff-analyzer](https://github.com/Ertibus/sudoku-diff-analyzer/releases/tag/Release)
Juos išskleidus ir atidarius jų direktoriją:
- Paleisti `run.bat` failą.
- __Arba__ per _cmd/powershell_ paleisti komandą: `./sudoku-diff-detector.exe ./res/sudoku_learn.json ./res/sudoku_test.json`


## 2 method. Compile and run
Requirements:
- Rustup

[Download](https://www.rust-lang.org/tools/install?platform_override=win) and install rust. 

Po instaliacijos, atidaryti _cmd_ ir toje pačioje direktorijoje kur ir `Cargo.toml` failas, ir paleisti komanda:
`cargo run ./res/sudoku_learn.json ./res/sudoku_test.json` (Gali užtrukti minutę)

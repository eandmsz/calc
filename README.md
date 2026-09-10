# Calc

## Stateful scientific calculator focusing on intuitive user experience

- Written purely in Rust. No dependencies or wrappers for C or Python
- Focusing on simplicity and ease-of-use: Does what you would expect from a calculator, nothing more or less
- Logical and aesthetic layout: Superscript & subscript display, predefined themes, configurable fonts and shapes
- Stateful operation for an intuitive workflow: can repeat the last operation or use backspace to delete backwards
- Hackable: Button layout, theme colors (by editing config.toml)
- Basic operations use decimal arithmetic, so `0.1 + 0.2 - 0.3` or `0.3 mod 0.1` result in exactly `0`
- Transcendental functions (trigonometry, exponentials, logarithms, roots) use IEEE 754 f64 (gives 15–17 significant decimal digits precision)

## Recommended fonts to install:

 - Cupertino: SF Pro Display
 - Redmond: Segoe UI
 - Wolfenstein: zilverstone eYe/FS
 - Army: ArmyChalk
 - Cosmic: Open Sans
 - Texas: Consolas
 - Tokyo: Noto Sans
 - Cyberpunk: Adwaita Mono
 - Plastic: Comfortaa
 - Crystal: Cantarell
 - Barbie:
 - Emerald: Cambria
 - Flat: Trebuchet MS
 - Matrix: JetBrains Mono"
 - Sandstone: 

If a font is  missing, the default general system font will be used or the one you select

## Out of scope:

- Arbitrary "infinite" precision arithmetic (Windows Calculator has it)
- Integral, derivative, lim, combinations (nCr), permutations (nPr), Fibonacci function
- Complex numbers and imaginary units
- Programmer's operations: bitshift, binary, hexadecimal calculations
- Economic and statistics calculations: mean, standard deviation, sum of squares
- Graphing calculations
- Date, Currency, Unit conversions (we want to keep this tool to be 100% offline)
- Area, perimeter, volume, surface formulas
- Physics, chemistry formulas/constants

## License

GPL-3.0-only. See [LICENSE](LICENSE).

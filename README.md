# Calc - a scientific calculator focusing on user experience

## Why we need a better calculator for Linux:
 Because none of the Linux calculators (Kcalc, Gnome Calculator, Galculator, MATE Calculator) match the macOS or Windows calculator's user experience. They are all trying to fit in into their desktop environment and therefore using the standard (Qt/GTK) buttons, radio buttons, drop-down menus etc. This does not give a good user experience in most cases and I'm tired of raising these issues to them which will never get fixed because it would either require too much work or it goes straigth against their design philoshopy.
 A few examples:
 - None of the Linux calculators above are stateful which means e.g. you cannot simply repeat the last operation by pressing equals sign repeatedly (as you would do on a real calculator)
 - All of the Linux calculators above let you to enter malformed expressions e.g.: `8++1` which will result in an error.
 - Kcalc solves `sqrt(-2)` resulting in `1,4142135623730950488i` and even though it is mathematically correct (so it won't be fixed) I'm pretty sure most of the users expect an error and not a complex number. By the way: you've missed that `i` at the end, haven't you?
 - Gnome Calculator: you press a number on your keypad and it might not be entered if the text area of the calculator was not selected, so you need to reach for the mouse to click where you want to enter that number
 
 - 
 - 

## Features:

- Written purely in Rust. No dependencies or wrappers for C or Python
- Focusing on simplicity and ease-of-use: Does what you would expect from a calculator, nothing more or less
- Logical and aesthetic layout: Superscript & subscript display, predefined themes, configurable fonts and shapes
- Stateful operation for an intuitive workflow: can repeat the last operation or use backspace to delete backwards
- Hackable: Button layout by editing config.toml
- Basic operations use decimal arithmetic, so `0.1 + 0.2 - 0.3` or `0.3 mod 0.1` result in exactly `0`
- Transcendental functions (trigonometry, exponentials, logarithms, roots) use IEEE 754 f64 (gives 15–17 significant decimal digits precision)

## Recommended fonts to install:

 - Cupertino: SF Pro Display
 - Redmond: Segoe UI
 - Wolfenstein: zilverstone eYe/FS
 - Army: ArmyChalk
 - Cosmic: Open Sans
 - Texas: Consolas
 - Cyberpunk: Adwaita Mono
 - Plastic: Comfortaa
 - Crystal: Cantarell
 - Barbie:
 - Emerald: Cambria
 - Flat: Trebuchet MS
 - Matrix: JetBrains Mono
 - Sandstone: Roboto Slab
 - Monokai: Noto Sans Math

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

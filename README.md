# Calc

## An easy-to-use stateful scientific calculator focusing on intuitive user experience

- Written purely in Rust. No dependencies or wrappers for C or Python
- Focusing on simplicity and ease-of-use: Does what you would expect from a calculator, nothing more, nothing less
- Logical and aesthetic layout: Superscript & subscript support, different colors and shapes via predefined themes, configurable fonts and button shapes
- Stateful operation for an intuitive workflow: e.g. you can repeat the last operation or use backspace to delete backwards
- Hackable: Button layout, theme colors (by editing config.toml)
- Basic operations use decimal arithmetic, so `0.1 + 0.2 - 0.3` or `0.3 mod 0.1` result in exactly `0`
- Transcendental functions (trigonometry, exponentials, logarithms, roots) use IEEE 754 f64 which gives 15–17 significant decimal digits precision

## Out of scope:

- Arbitrary "infinite" precision arithmetic (the decimal type is a
  fixed 18 digits — enough that the 15 on screen are always right, not
  enough to hold a number of any size you like)
- Integral, derivative, lim, combinations (nCr), permutations (nPr), Fibonacci function
- Complex numbers and imaginary units
- Programmer's operations: bitshift, binary, hexadecimal calculations
- Economic and statistics calculations: mean, standard deviation, sum of squares
- Graphing calculations
- Date, Currency, Unit conversions (currency would need a data provider and we want to keep this tool to be 100% offline)
- Area, perimeter, volume, surface formulas
- Physics, chemistry formulas/constants

## License

GPL-3.0-only. See [LICENSE](LICENSE).

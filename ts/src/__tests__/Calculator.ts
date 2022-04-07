import { Calculator } from "../Calculator";

/**
 * TODO:
 * * parseAndSum('') -> 0
 * * parseAndSum('1') -> 1
 * * parseAndSum('abc') -> 0
 * * parseAndSum('1,2,A,4') -> 7
 * * parseAndSum('1\n2,3') -> 6
 * * parseAndSum('//;\n1;2;3;ref') -> 6
 * * parseAndSum('-1,2,3') -> 'Negatives not allowed'
 * * parseAndSum('100,1002,1') -> 101
 * * parseAndSum('//[;][,]\n1;2,3') -> 6
 * * parseAndSum('//[**][^^]\n2**2^^2') -> 6
 */
describe("StringCalculator", () => {
  let calculator: Calculator;

  beforeEach(() => {
    calculator = new Calculator();
  });

  it("Should return 0 if the input is empty", () => {
    expect(calculator.parseAndSum("")).toBe(0);
  });

  it("Should parse the number if there is only one", () => {
    expect(calculator.parseAndSum("1")).toBe(1);
  });

  it("Should ignore all non numericals characters", () => {
    expect(calculator.parseAndSum("abc")).toBe(0);
  });

  it("Should sum the numbers separated by commas", () => {
    expect(calculator.parseAndSum("1,2")).toBe(3);
    expect(calculator.parseAndSum("1,2,5")).toBe(8);
    expect(calculator.parseAndSum("1,2,a")).toBe(3);
  });

  it("Should sum the numbers separated by new lines and commas", () => {
    expect(calculator.parseAndSum("1\n2")).toBe(3);
    expect(calculator.parseAndSum("1\n2,3")).toBe(6);
    expect(calculator.parseAndSum("1,2\n3,4")).toBe(10);
    expect(calculator.parseAndSum("1,\n")).toBe(1);
  });

  it("Should sum the numbers separated by custom delimiters with //delimiter\\n syntax", () => {
    expect(calculator.parseAndSum("//;\n1;2;3")).toBe(6);
  });

  it("Shouldn't allow negative numbers", () => {
    expect(() => calculator.parseAndSum("1,-2,-3")).toThrowError(/.*/);
  });

  it("Should include the found negative numbers in the error", () => {
    expect(() => calculator.parseAndSum("-1,2,-3")).toThrowError(/-1.*-3/);
  });
});

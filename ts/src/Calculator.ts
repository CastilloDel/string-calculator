export class Calculator {
  readonly defaultDelimiter = /[,\n]/;
  private negativeNumbers: number[] = [];

  public parseAndSum(input: string): number {
    this.negativeNumbers = [];
    const [delimiter, rest] = this.processDelimiter(input);
    const result = this.parseAndSumWithDelimiter(rest, delimiter);
    if (this.negativeNumbers.length !== 0) {
      throw `Negatives not allowed: ${this.negativeNumbers.join(", ")}`;
    }
    return result;
  }

  private parseAndSumWithDelimiter(input: string, delimiter: RegExp): number {
    const [firstNumber, rest] = this.splitFirstNumber(input, delimiter);
    this.storeIfNegative(firstNumber);
    if (rest === "") return firstNumber;
    return firstNumber + this.parseAndSumWithDelimiter(rest, delimiter);
  }

  private storeIfNegative(number: number) {
    if (number < 0) {
      this.negativeNumbers.push(number);
    }
  }

  private splitFirstNumber(input: string, delimiter: RegExp): [number, string] {
    const firstDelimiter = input.search(delimiter);
    if (firstDelimiter === -1) {
      return [this.parseNumber(input), ""];
    }
    const firstNumber = this.parseNumber(input.substring(0, firstDelimiter));
    const rest = input.substring(firstDelimiter + 1);
    return [firstNumber, rest];
  }

  private parseNumber(number: string) {
    const parsedNumber = parseInt(number);
    if (isNaN(parsedNumber)) return 0;
    return parsedNumber;
  }

  private processDelimiter(input: string): [RegExp, string] {
    const delimiter = input.match(/^\/\/(.*?)\n/);
    if (delimiter) {
      const input_without_delimiter = input.substring(input.indexOf("\n") + 1);
      return [new RegExp(delimiter[1]), input_without_delimiter];
    }
    return [this.defaultDelimiter, input];
  }
}

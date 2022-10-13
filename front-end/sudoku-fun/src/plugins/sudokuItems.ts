export interface SudokuItem {
  value: number;
  block: number;
  horizontal: number;
  vertical: number;
}

export interface SudokuItems {
  sudokuItems: Array<SudokuItem>;
}

export const SudokuValues = [1, 2, 3, 4, 5, 6, 7, 8, 9];
export const rowAndCols = [0, 1, 2, 3, 4, 5, 6, 7, 8];

export async function generateSudokuItems(): Promise<Array<SudokuItem>> {
  let sudokuItems = new Array<SudokuItem>();
  let block = 0;
  rowAndCols.forEach((_, index) => {
    let currentBlock = 0;
    let blockCounter = -1;
    let cols = [...rowAndCols];
    cols.forEach((_, j) => {
      blockCounter += 1;
      let itemIndex = index * 9 + j;
      let horizontal = index;
      let vertical = j;
      let itemBlock = block + currentBlock;
      if (blockCounter == 2) {
        blockCounter = -1;
        currentBlock += 1;
        if ((itemIndex + 1) % 27 == 0) {
          block += 3;
        }
      }
      let sudokuItem = {
        index: itemIndex,
        horizontal,
        vertical,
        block: itemBlock,
        value: 0,
      };
      sudokuItems.push(sudokuItem);
    });
  });
  return sudokuItems;
}

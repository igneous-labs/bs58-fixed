import {
  zeroLast,
  zeroLastRef,
  zeroLastObj,
  zeroLastOptObj,
  zeroLastVecObj,
  zeroLastVec,
  zeroLastOpt,
  type Bs58Pk,
} from "bs58-fixed-wasm-consumer";
import { describe, expect, it } from "vitest";

const ALL_ONES: Bs58Pk = "4vJ9JU1bJJE96FWSJKvHsmmFADCg4gpZQff4P3bkLKi";

const ALL_ONES_ZERO_LAST: Bs58Pk =
  "4vJ9JU1bJJE96FWSJKvHsmmFADCg4gpZQff4P3bkLKh";

const ALL_ZEROS: Bs58Pk = "11111111111111111111111111111111";

describe("allOnes", () => {
  it("zeroLast", () => {
    expect(zeroLast(ALL_ONES)).toStrictEqual(ALL_ONES_ZERO_LAST);
  });

  it("zeroLastRef", () => {
    expect(zeroLastRef(ALL_ONES)).toStrictEqual(ALL_ONES_ZERO_LAST);
  });

  it("zeroLastOpt none", () => {
    expect(zeroLastOpt(null)).toStrictEqual(ALL_ZEROS);
  });

  it("zeroLastOpt some", () => {
    expect(zeroLastOpt(ALL_ONES)).toStrictEqual(ALL_ONES_ZERO_LAST);
  });

  it("zeroLastVec", () => {
    expect(zeroLastVec([ALL_ONES])).toStrictEqual([ALL_ONES_ZERO_LAST]);
  });

  it("zeroLastObj", () => {
    expect(
      zeroLastObj({
        arg: ALL_ONES,
      })
    ).toStrictEqual({
      arg: ALL_ONES_ZERO_LAST,
    });
  });

  it("zeroLastOptObj none", () => {
    expect(
      zeroLastOptObj({
        arg: undefined,
      })
    ).toStrictEqual({
      arg: ALL_ZEROS,
    });
  });

  it("zeroLastOptObj some", () => {
    expect(
      zeroLastOptObj({
        arg: ALL_ONES,
      })
    ).toStrictEqual({
      arg: ALL_ONES_ZERO_LAST,
    });
  });

  it("zeroLastVecObj", () => {
    expect(zeroLastVecObj({ arg: [ALL_ONES] })).toStrictEqual({
      arg: [ALL_ONES_ZERO_LAST],
    });
  });
});

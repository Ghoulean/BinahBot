import { Localization } from "@ghoulean/ruina-common";
import { DataAccessor } from "../../src/accessor/data_accessor";

const WEIGHT_OF_SIN_EN_PAGE_NAME = "The Weight of Sin";
const WEIGHT_OF_SIN_PAGE_ID = "LongBird_Sin";
const BAD_PAGE_NAME = "PageNameThatDoesNotExist";
const BAD_PAGE_ID = "PageIdThatDoesNotExist";

let dataAccessor: DataAccessor;

beforeEach(() => {
    dataAccessor = new DataAccessor();
});

test("should return lookup result when given page name and locale", () => {
    expect(
        dataAccessor.lookup(WEIGHT_OF_SIN_EN_PAGE_NAME, Localization.ENGLISH)
    ).toMatchSnapshot();
});

test("should throw error when given bad lookup request", () => {
    expect(() => {
        dataAccessor.lookup(BAD_PAGE_NAME, Localization.ENGLISH);
    }).toThrow();
});

test("should return decorated abno page result when given page id and locale", () => {
    expect(
        dataAccessor.getDecoratedAbnoPage(
            WEIGHT_OF_SIN_PAGE_ID,
            Localization.ENGLISH
        )
    ).toMatchSnapshot();
});

test("should throw error when given bad decorated abno page request", () => {
    expect(() => {
        dataAccessor.getDecoratedAbnoPage(BAD_PAGE_ID, Localization.ENGLISH);
    }).toThrow();
});

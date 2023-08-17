import { Chapter, Localization } from "@ghoulean/ruina-common";
import { DataAccessor } from "../../src/accessor/data_accessor";
import { LorAutocomplete } from "../../src/command/lor_autocomplete";
import { CommandResult } from "../../src/model/command_result";
import { Request } from "../../src/model/request";
import { ABNO_LOOKUP_RESULT } from "../resources/lookup_results";

const REQUEST: Request = {
    command: "lor",
    commandArgs: {
        query: "",
    },
    interactionToken: "",
    locale: Localization.ENGLISH,
    chapter: Chapter.IMPURITAS_CIVITATIS,
};

const LARGE_AUTOCOMPLETE_SEARCH: number = 15;

const AUTOCOMPLETE_RESULT: string[] = Array(LARGE_AUTOCOMPLETE_SEARCH).fill(
    "result"
);

const mockDataAccessor = new (<new () => DataAccessor>(
    DataAccessor
))() as jest.Mocked<DataAccessor>;

let lorAutocomplete: LorAutocomplete;

beforeEach(() => {
    lorAutocomplete = new LorAutocomplete(mockDataAccessor);
});

test("should return autocomplete results", () => {
    mockDataAccessor.autocomplete = jest.fn();
    mockDataAccessor.lookup = jest.fn();

    mockDataAccessor.autocomplete.mockReturnValueOnce(AUTOCOMPLETE_RESULT);
    mockDataAccessor.lookup.mockReturnValueOnce({
        ...ABNO_LOOKUP_RESULT,
        displayQuery: "displayQuery"
    }).mockReturnValue(ABNO_LOOKUP_RESULT);

    const commandResult: CommandResult = lorAutocomplete.autocomplete(REQUEST);

    expect(commandResult.success).toBe(true);
    expect((commandResult.payload as string[]).length).toBeLessThan(
        LARGE_AUTOCOMPLETE_SEARCH
    );
    expect(commandResult).toMatchSnapshot();
});

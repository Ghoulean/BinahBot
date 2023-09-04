import { Chapter, Localization } from "@ghoulean/ruina-common";
import { LorAutocomplete } from "../../src/command/lor_autocomplete";
import { AutocompleteResult, CommandResult } from "../../src/model/command_result";
import { Request } from "../../src/model/request";
import { AutocompleteRouter } from "../../src/router/autocomplete_router";

const BASE_REQUEST: Request = {
    command: "",
    commandArgs: {},
    interactionToken: "interactionToken",
    locale: Localization.ENGLISH,
    chapter: Chapter.IMPURITAS_CIVITATIS,
};

const COMMAND_RESULT: AutocompleteResult = {
    success: true,
};

const mockLorAutocomplete = new (<new () => LorAutocomplete>(
    LorAutocomplete
))() as jest.Mocked<LorAutocomplete>;

let autocompleteRouter: AutocompleteRouter;

beforeEach(() => {
    autocompleteRouter = new AutocompleteRouter(mockLorAutocomplete);
});

test("should route lor autocomplete to lor autocomplete and bubble result", () => {
    mockLorAutocomplete.autocomplete = jest.fn();

    const request: Request = {
        ...BASE_REQUEST,
        command: "lor",
    };

    mockLorAutocomplete.autocomplete.mockReturnValueOnce(COMMAND_RESULT);

    expect(autocompleteRouter.routeRequest(request)).toBe(COMMAND_RESULT);
});

test("should return failure when receive unrecognized command", () => {
    const request: Request = {
        ...BASE_REQUEST,
        command: "unrecognized",
    };

    expect(autocompleteRouter.routeRequest(request).success).toBe(false);
});

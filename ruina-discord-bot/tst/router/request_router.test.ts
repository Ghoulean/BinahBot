import { Chapter, Localization } from "@ghoulean/ruina-common";
import { LorAutocomplete } from "../../src/command/lor_autocomplete";
import { LorCommand } from "../../src/command/lor_command";
import { CommandResult } from "../../src/model/command_result";
import { Request } from "../../src/model/request";
import { RequestRouter } from "../../src/router/request_router";

const mockLorCommand = new (<new () => LorCommand>(
    LorCommand
))() as jest.Mocked<LorCommand>;
const mockLorAutocomplete = new (<new () => LorAutocomplete>(
    LorAutocomplete
))() as jest.Mocked<LorAutocomplete>;

const BASE_REQUEST: Request = {
    command: "",
    commandArgs: {},
    interactionToken: "interactionToken",
    locale: Localization.ENGLISH,
    chapter: Chapter.IMPURITAS_CIVITATIS,
};

const COMMAND_RESULT: CommandResult = {
    success: true,
};

let requestRouter: RequestRouter;

beforeEach(() => {
    requestRouter = new RequestRouter(mockLorCommand, mockLorAutocomplete);
});

test("should route lor commands to lor command and bubble result", () => {
    mockLorCommand.invoke = jest.fn();

    const request: Request = {
        ...BASE_REQUEST,
        command: "lor",
    };

    mockLorCommand.invoke.mockReturnValueOnce(COMMAND_RESULT);

    expect(requestRouter.routeRequest(request)).toBe(COMMAND_RESULT);
});

test("should route lor autocomplete to lor autocomplete and bubble result", () => {
    mockLorAutocomplete.autocomplete = jest.fn();

    const request: Request = {
        ...BASE_REQUEST,
        command: "lor",
        autocomplete: true,
    };

    mockLorAutocomplete.autocomplete.mockReturnValueOnce(COMMAND_RESULT);

    expect(requestRouter.routeRequest(request)).toBe(COMMAND_RESULT);
});

test("should return failure when receive unrecognized command", () => {
    const request: Request = {
        ...BASE_REQUEST,
        command: "unrecognized",
    };

    expect(requestRouter.routeRequest(request).success).toBe(false);
});

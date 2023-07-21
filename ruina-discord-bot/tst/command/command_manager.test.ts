import { Chapter, Localization } from "@ghoulean/ruina-common";
import { CommandManager } from "../../src/command/command_manager";
import { LorCommand } from "../../src/command/lor_command";
import { CommandResult } from "../../src/model/command_result";
import { Request } from "../../src/model/request";

const mockLorCommand = new (<new () => LorCommand>(
    LorCommand
))() as jest.Mocked<LorCommand>;

const BASE_REQUEST: Request = {
    command: "",
    commandArgs: {},
    interactionToken: "interactionToken",
    locale: Localization.ENGLISH,
    chapter: Chapter.IMPURITAS_CIVITATIS,
};

const COMMAND_RESULT: CommandResult = {
    success: false,
};

let commandManager: CommandManager;

beforeEach(() => {
    commandManager = new CommandManager(mockLorCommand);
});

test("should route lor commands to lor and bubble result", () => {
    mockLorCommand.invoke = jest.fn();

    const request = {
        ...BASE_REQUEST,
        command: "lor",
    };

    mockLorCommand.invoke.mockReturnValueOnce(Promise.resolve(COMMAND_RESULT));

    commandManager.routeRequest(request).then((value: CommandResult) => {
        expect(value).toBe(COMMAND_RESULT);
    });
});

test("should return failure when receive unrecognized command", () => {
    const request = {
        ...BASE_REQUEST,
        command: "unrecognized",
    };

    commandManager.routeRequest(request).then((value: CommandResult) => {
        expect(value.success).toEqual(false);
    });
});

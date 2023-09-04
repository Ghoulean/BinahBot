import { Chapter, Localization } from "@ghoulean/ruina-common";
import { BinahBotCommand } from "../../src/command/binahbot_command";
import { LorCommand } from "../../src/command/lor_command";
import { CommandResult } from "../../src/model/command_result";
import { Request } from "../../src/model/request";
import { CommandRouter } from "../../src/router/command_router";

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

const mockLorCommand = new (<new () => LorCommand>(
    LorCommand
))() as jest.Mocked<LorCommand>;
const mockBinahBotCommand = new (<new () => BinahBotCommand>(
    BinahBotCommand
))() as jest.Mocked<BinahBotCommand>;

let commandRouter: CommandRouter;

beforeEach(() => {
    commandRouter = new CommandRouter(mockLorCommand, mockBinahBotCommand);
});

test("should route lor commands to lor command and bubble result", () => {
    mockLorCommand.invoke = jest.fn();

    const request: Request = {
        ...BASE_REQUEST,
        command: "lor",
    };

    mockLorCommand.invoke.mockReturnValueOnce(COMMAND_RESULT);

    expect(commandRouter.routeRequest(request)).toBe(COMMAND_RESULT);
});

test("should route binahbot commands to binahbot command and bubble result", () => {
    mockBinahBotCommand.invoke = jest.fn();

    const request: Request = {
        ...BASE_REQUEST,
        command: "binahbot",
    };

    mockBinahBotCommand.invoke.mockReturnValueOnce(COMMAND_RESULT);

    expect(commandRouter.routeRequest(request)).toBe(COMMAND_RESULT);
});

test("should return failure when receive unrecognized command", () => {
    const request: Request = {
        ...BASE_REQUEST,
        command: "unrecognized",
    };

    expect(commandRouter.routeRequest(request).success).toBe(false);
});

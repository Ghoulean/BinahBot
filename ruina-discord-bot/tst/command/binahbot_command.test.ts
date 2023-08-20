import { Chapter, Localization } from "@ghoulean/ruina-common";
import { BinahBotCommand } from "../../src/command/binahbot_command";
import { Request } from "../../src/model/request";

const REQUEST: Request = {
    command: "binahbot",
    commandArgs: {},
    interactionToken: "it",
    locale: Localization.ENGLISH,
    chapter: Chapter.IMPURITAS_CIVITATIS,
};

const CLIENT_ID: string = "clientId";

let binahbotCommand: BinahBotCommand;

beforeEach(() => {
    binahbotCommand = new BinahBotCommand(CLIENT_ID);
});

test("should return about me", () => {
    expect(binahbotCommand.invoke(REQUEST)).toMatchSnapshot();
});

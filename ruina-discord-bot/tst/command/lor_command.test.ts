import {
    AbnoPageSelectType,
    Chapter,
    DecoratedAbnoPage,
    Floor,
    Localization,
    LookupResult,
    PageType,
} from "@ghoulean/ruina-common";
import { DataAccessor } from "../../src/accessor/data_accessor";
import { DiscordAccessor } from "../../src/accessor/discord_accessor";
import { LorCommand } from "../../src/command/lor_command";
import { CommandResult } from "../../src/model/command_result";
import { DiscordEmbed } from "../../src/model/discord/discord_embed";
import { Request } from "../../src/model/request";
import { AbnoToEmbedTransformer } from "../../src/transformers/abno_to_embed_transformer";

const COMMAND_QUERY: string = "commandQuery";
const COMMAND_ABNO_PAGE_ID: string = "commandAbnoPageId";
const INTERACTION_TOKEN: string = "interactionToken";

const REQUEST: Request = {
    command: "lor",
    commandArgs: {
        query: COMMAND_QUERY,
    },
    interactionToken: INTERACTION_TOKEN,
    locale: Localization.ENGLISH,
    chapter: Chapter.IMPURITAS_CIVITATIS,
};

const ABNO_LOOKUP_RESULT: LookupResult = {
    query: COMMAND_QUERY,
    chapter: Chapter.LIBRARY_OF_RUINA,
    locale: Localization.ENGLISH,
    pageType: PageType.ABNO_PAGE,
    pageId: COMMAND_ABNO_PAGE_ID,
};

const DECORATED_ABNO_PAGE: DecoratedAbnoPage = {
    locale: Localization.ENGLISH,
    name: "",
    description: "",
    flavorText: "",
    imagePath: "",
    nameId: "",
    floor: Floor.NONE,
    chapter: Chapter.LIBRARY_OF_RUINA,
    emoLevel: 0,
    emotionSign: 0,
    emotionRate: 0,
    targetType: AbnoPageSelectType.SELECT_ONE,
    scriptId: "",
};

const DISCORD_EMBED: DiscordEmbed = {
    fields: [],
};

const mockDataAccessor = new (<new () => DataAccessor>(
    DataAccessor
))() as jest.Mocked<DataAccessor>;
const mockDiscordAccessor = new (<new () => DiscordAccessor>(
    DiscordAccessor
))() as jest.Mocked<DiscordAccessor>;
const mockAbnoToEmbedTransformer = new (<new () => AbnoToEmbedTransformer>(
    AbnoToEmbedTransformer
))() as jest.Mocked<AbnoToEmbedTransformer>;

let lorCommand: LorCommand;

beforeEach(() => {
    lorCommand = new LorCommand(
        mockDataAccessor,
        mockDiscordAccessor,
        mockAbnoToEmbedTransformer
    );
});

test("should send abno page embed to Discord on invocation with abno page", () => {
    mockDataAccessor.lookup = jest.fn();
    mockDataAccessor.getDecoratedAbnoPage = jest.fn();
    mockAbnoToEmbedTransformer.transform = jest.fn();
    mockDiscordAccessor.writeResponse = jest.fn();

    mockDataAccessor.lookup.mockReturnValueOnce(ABNO_LOOKUP_RESULT);
    mockDataAccessor.getDecoratedAbnoPage.mockReturnValueOnce(
        DECORATED_ABNO_PAGE
    );
    mockAbnoToEmbedTransformer.transform.mockReturnValueOnce(DISCORD_EMBED);
    mockDiscordAccessor.writeResponse.mockReturnValueOnce(Promise.resolve());

    lorCommand.invoke(REQUEST).then((response: CommandResult) => {
        expect(response).toEqual({
            success: true,
        });
    });
});

test("should fail when abno page can't be found", () => {
    mockDataAccessor.lookup = jest.fn();
    mockDataAccessor.getDecoratedAbnoPage = jest.fn();

    mockDataAccessor.lookup.mockReturnValueOnce(ABNO_LOOKUP_RESULT);
    mockDataAccessor.getDecoratedAbnoPage.mockImplementationOnce(() => {
        throw new Error();
    });

    lorCommand.invoke(REQUEST).then((response: CommandResult) => {
        expect(response.success).toBe(false);
    });
});

test("should fail when discord fails sending embed", () => {
    mockDataAccessor.lookup = jest.fn();
    mockDataAccessor.getDecoratedAbnoPage = jest.fn();
    mockAbnoToEmbedTransformer.transform = jest.fn();
    mockDiscordAccessor.writeResponse = jest.fn();

    mockDataAccessor.lookup.mockReturnValueOnce(ABNO_LOOKUP_RESULT);
    mockDataAccessor.getDecoratedAbnoPage.mockReturnValueOnce(
        DECORATED_ABNO_PAGE
    );
    mockAbnoToEmbedTransformer.transform.mockReturnValueOnce(DISCORD_EMBED);
    mockDiscordAccessor.writeResponse.mockImplementationOnce(() => {
        throw new Error();
    });

    lorCommand.invoke(REQUEST).then((response: CommandResult) => {
        expect(response.success).toBe(false);
    });
});

test("should fail when receive unsupported page type", () => {
    mockDataAccessor.lookup = jest.fn();

    mockDataAccessor.lookup.mockReturnValueOnce({
        ...ABNO_LOOKUP_RESULT,
        pageType: PageType.PASSIVE,
    });
    mockDataAccessor.getDecoratedAbnoPage.mockReturnValueOnce(
        DECORATED_ABNO_PAGE
    );

    lorCommand.invoke(REQUEST).then((response: CommandResult) => {
        expect(response.success).toBe(false);
    });
});

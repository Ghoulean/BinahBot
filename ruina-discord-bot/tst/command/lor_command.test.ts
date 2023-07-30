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
import { LorCommand } from "../../src/command/lor_command";
import { DiscordEmbed } from "../../src/model/discord/discord_embed";
import { Request } from "../../src/model/request";
import { EmbedTransformer } from "../../src/transformers/embed_transformer";

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

const REQUEST_WITH_LOCALE: Request = {
    ...REQUEST,
    commandArgs: {
        query: COMMAND_QUERY,
        locale: "en",
    },
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
const embedTransformer = new (<new () => EmbedTransformer>(
    EmbedTransformer
))() as jest.Mocked<EmbedTransformer>;

let lorCommand: LorCommand;

beforeEach(() => {
    lorCommand = new LorCommand(mockDataAccessor, embedTransformer);
});

test("should send abno page embed to Discord on invocation with abno page", () => {
    mockDataAccessor.lookup = jest.fn();
    mockDataAccessor.getDecoratedAbnoPage = jest.fn();
    embedTransformer.transformAbnoPage = jest.fn();

    mockDataAccessor.lookup.mockReturnValueOnce(ABNO_LOOKUP_RESULT);
    mockDataAccessor.getDecoratedAbnoPage.mockReturnValueOnce(
        DECORATED_ABNO_PAGE
    );
    embedTransformer.transformAbnoPage.mockReturnValueOnce(DISCORD_EMBED);

    expect(lorCommand.invoke(REQUEST_WITH_LOCALE)).toEqual({
        success: true,
        payload: DISCORD_EMBED,
    });
});

test("should fail when lookup returns abno page but abno page can't be found", () => {
    mockDataAccessor.lookup = jest.fn();
    mockDataAccessor.getDecoratedAbnoPage = jest.fn();

    mockDataAccessor.lookup.mockReturnValueOnce(ABNO_LOOKUP_RESULT);
    mockDataAccessor.getDecoratedAbnoPage.mockImplementationOnce(() => {
        throw new Error();
    });

    expect(lorCommand.invoke(REQUEST).success).toBe(false);
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

    expect(lorCommand.invoke(REQUEST).success).toBe(false);
});

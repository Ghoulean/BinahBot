import {
    Chapter,
    Localization,
    PageType
} from "@ghoulean/ruina-common";
import { DataAccessor } from "../../src/accessor/data_accessor";
import { LorCommand } from "../../src/command/lor_command";
import { DisambiguationResults } from "../../src/model/disambiguation_result";
import { DiscordEmbed } from "../../src/model/discord/discord_embed";
import { Request } from "../../src/model/request";
import { EmbedTransformer } from "../../src/transformers/embed_transformer";
import {
    DECORATED_ABNO_PAGE,
    BASE_DECORATED_COMBAT_PAGE,
} from "../resources/decorated_pages";
import {
    ABNO_LOOKUP_RESULT,
    COMBAT_LOOKUP_RESULT,
    DISAMBIGUATION_LOOKUP_RESULT,
} from "../resources/lookup_results";

const INTERACTION_TOKEN: string = "interactionToken";

const REQUEST: Request = {
    command: "lor",
    commandArgs: {
        query: "query",
    },
    interactionToken: INTERACTION_TOKEN,
    locale: Localization.ENGLISH,
    chapter: Chapter.IMPURITAS_CIVITATIS,
};

const REQUEST_WITH_LOCALE: Request = {
    ...REQUEST,
    commandArgs: {
        query: "query",
        locale: "en",
    },
};

const DISAMBIGUATION_RESULT: DisambiguationResults = {
    id: "",
    locale: Localization.KOREAN,
    query: "",
    lookupResults: [],
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

test("should send combat page embed to Discord on invocation with combat page", () => {
    mockDataAccessor.lookup = jest.fn();
    mockDataAccessor.getDecoratedCombatPage = jest.fn();
    embedTransformer.transformCombatPage = jest.fn();

    mockDataAccessor.lookup.mockReturnValueOnce(COMBAT_LOOKUP_RESULT);
    mockDataAccessor.getDecoratedCombatPage.mockReturnValueOnce(
        BASE_DECORATED_COMBAT_PAGE
    );
    embedTransformer.transformCombatPage.mockReturnValueOnce(DISCORD_EMBED);

    expect(lorCommand.invoke(REQUEST_WITH_LOCALE)).toEqual({
        success: true,
        payload: DISCORD_EMBED,
    });
});

test("should send disambiguation page embed to Discord on invocation with page that requires disambiguation", () => {
    mockDataAccessor.lookup = jest.fn();
    mockDataAccessor.getDisambiguationResult = jest.fn();
    embedTransformer.transformDisambiguationPage = jest.fn();

    mockDataAccessor.lookup.mockReturnValueOnce(DISAMBIGUATION_LOOKUP_RESULT);
    mockDataAccessor.getDisambiguationResult.mockReturnValueOnce(
        DISAMBIGUATION_RESULT
    );
    embedTransformer.transformDisambiguationPage.mockReturnValueOnce(
        DISCORD_EMBED
    );

    expect(lorCommand.invoke(REQUEST_WITH_LOCALE)).toEqual({
        success: true,
        payload: DISCORD_EMBED,
    });
});

test("should return no page found when lookup returns no page", () => {
    mockDataAccessor.lookup = jest.fn();
    embedTransformer.noResultsFoundEmbed = jest.fn();

    mockDataAccessor.lookup.mockImplementationOnce(() => {
        throw new Error();
    });
    embedTransformer.noResultsFoundEmbed.mockReturnValueOnce(DISCORD_EMBED);

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

test("should fail when lookup returns combat page but combat page can't be found", () => {
    mockDataAccessor.lookup = jest.fn();
    mockDataAccessor.getDecoratedCombatPage = jest.fn();

    mockDataAccessor.lookup.mockReturnValueOnce(COMBAT_LOOKUP_RESULT);
    mockDataAccessor.getDecoratedCombatPage.mockImplementationOnce(() => {
        throw new Error();
    });

    expect(lorCommand.invoke(REQUEST).success).toBe(false);
});

test("should fail when lookup returns disambiguation page but disambiguation page can't be found", () => {
    mockDataAccessor.lookup = jest.fn();
    mockDataAccessor.getDisambiguationResult = jest.fn();

    mockDataAccessor.lookup.mockReturnValueOnce(DISAMBIGUATION_LOOKUP_RESULT);
    mockDataAccessor.getDisambiguationResult.mockImplementationOnce(() => {
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

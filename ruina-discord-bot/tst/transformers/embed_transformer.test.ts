import {
    Chapter,
    DecoratedAbnoPage,
    Localization,
    PageType,
    Rarity,
} from "@ghoulean/ruina-common";
import { DataAccessor } from "../../src/accessor/data_accessor";
import { DisambiguationResults } from "../../src/model/disambiguation_result";
import { DiscordEmbed } from "../../src/model/discord/discord_embed";
import { EmbedTransformer } from "../../src/transformers/embed_transformer";
import { DiscordEmbedColors } from "../../src/util/constants";
import {
    BASE_DECORATED_COMBAT_PAGE,
    DECORATED_ABNO_PAGE,
} from "../resources/decorated_pages";
import { DISAMBIGUATION_RESULTS } from "../resources/lookup_results";

const DEFAULT_REQUEST_LOCALE: Localization = Localization.ENGLISH;

const POSITIVE_DECORATED_ABNO_PAGE: DecoratedAbnoPage = {
    ...DECORATED_ABNO_PAGE,
    emotionRate: 1,
    emotionSign: 1,
};

const NEGATIVE_DECORATED_ABNO_PAGE: DecoratedAbnoPage = {
    ...DECORATED_ABNO_PAGE,
    emotionSign: -1,
    emotionRate: -1,
};

const NO_RESULTS_FOUND_QUERY: string = "query";

const S3_BUCKET_NAME: string = "s3bucketname";

let embedTransformer: EmbedTransformer;

beforeEach(() => {
    embedTransformer = new EmbedTransformer(S3_BUCKET_NAME);
});

test("should transform postive decorated abno page to Discord embed", () => {
    const discordEmbed: DiscordEmbed = embedTransformer.transformAbnoPage(
        POSITIVE_DECORATED_ABNO_PAGE,
        DEFAULT_REQUEST_LOCALE
    );
    expect(discordEmbed.color).toBe(DiscordEmbedColors.AWAKENING_ABNO_PAGE);
    expect(discordEmbed).toMatchSnapshot();
});

test("should transform negative decorated abno page to Discord embed", () => {
    const discordEmbed: DiscordEmbed = embedTransformer.transformAbnoPage(
        NEGATIVE_DECORATED_ABNO_PAGE,
        DEFAULT_REQUEST_LOCALE
    );
    expect(discordEmbed.color).toBe(DiscordEmbedColors.BREAKDOWN_ABNO_PAGE);
    expect(discordEmbed).toMatchSnapshot();
});

test.each([
    {
        descriptor: "rarity",
        combatPage: {
            ...BASE_DECORATED_COMBAT_PAGE,
            rarity: Rarity.PAPERBACK,
        },
        expectedColor: DiscordEmbedColors.PAPERBACK_RARITY,
    },
    {
        descriptor: "rarity",
        combatPage: {
            ...BASE_DECORATED_COMBAT_PAGE,
            rarity: Rarity.HARDCOVER,
        },
        expectedColor: DiscordEmbedColors.HARDCOVER_RARITY,
    },
    {
        descriptor: "rarity",
        combatPage: {
            ...BASE_DECORATED_COMBAT_PAGE,
            rarity: Rarity.LIMITED,
        },
        expectedColor: DiscordEmbedColors.LIMITED_RARITY,
    },
    {
        descriptor: "rarity",
        combatPage: {
            ...BASE_DECORATED_COMBAT_PAGE,
            rarity: Rarity.OBJET_D_ART,
        },
        expectedColor: DiscordEmbedColors.OBJET_D_ART_RARITY,
    },
])(
    "should transform decorated combat page to Discord embed (rarity: $rarity)",
    ({ combatPage, expectedColor }) => {
        const discordEmbed: DiscordEmbed = embedTransformer.transformCombatPage(
            combatPage,
            DEFAULT_REQUEST_LOCALE
        );
        expect(discordEmbed.color).toBe(expectedColor);
        expect(discordEmbed).toMatchSnapshot();
    }
);

test("should transform disambiguation page to Discord embed", () => {
    const discordEmbed: DiscordEmbed = embedTransformer.transformDisambiguationPage(
        DISAMBIGUATION_RESULTS,
        DEFAULT_REQUEST_LOCALE
    );
    expect(discordEmbed).toMatchSnapshot();
});

test("should return no results found embed", () => {
    expect(
        embedTransformer.noResultsFoundEmbed(
            NO_RESULTS_FOUND_QUERY,
            DEFAULT_REQUEST_LOCALE
        )
    ).toMatchSnapshot();
});

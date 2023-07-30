import {
    AbnoPageSelectType,
    Chapter,
    DecoratedAbnoPage,
    Floor,
    Localization,
} from "@ghoulean/ruina-common";
import { DiscordEmbed } from "../../src/model/discord/discord_embed";
import { EmbedTransformer } from "../../src/transformers/embed_transformer";
import { DiscordEmbedColors } from "../../src/util/constants";

const DEFAULT_REQUEST_LOCALE: Localization = Localization.ENGLISH;

const POSITIVE_DECORATED_ABNO_PAGE: DecoratedAbnoPage = {
    locale: Localization.ENGLISH,
    name: "TestAbnoPage",
    description: "TestDescription",
    flavorText: "TestFlavorText",
    imagePath: "test.png",
    nameId: "test",
    floor: Floor.NONE,
    chapter: Chapter.LIBRARY_OF_RUINA,
    emoLevel: 1,
    emotionRate: 1,
    emotionSign: 1,
    targetType: AbnoPageSelectType.SELECT_ONE,
    scriptId: "testScriptId",
};

const NEGATIVE_DECORATED_ABNO_PAGE: DecoratedAbnoPage = {
    ...POSITIVE_DECORATED_ABNO_PAGE,
    emotionSign: -1,
    emotionRate: -1,
};

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

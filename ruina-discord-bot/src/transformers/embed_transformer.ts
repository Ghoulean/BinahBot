import {
    DecoratedAbnoPage,
    DecoratedCombatPage,
    Localization,
    LookupResult,
    PageType,
    Rarity,
} from "@ghoulean/ruina-common";
import { DataAccessor } from "../accessor/data_accessor";
import { DisambiguationResults } from "../model/disambiguation_result";
import { DiscordEmbed } from "../model/discord/discord_embed";
import { DiscordEmbedColors } from "../util/constants";

export class EmbedTransformer {
    private readonly s3BucketName: string;

    constructor(s3BucketName: string) {
        this.s3BucketName = s3BucketName;
    }

    // TODO: localize via _requestLocale
    public transformAbnoPage(
        abno: DecoratedAbnoPage,
        _requestLocale: Localization
    ): DiscordEmbed {
        const embedColor: number =
            abno.emotionSign > 0
                ? DiscordEmbedColors.AWAKENING_ABNO_PAGE
                : DiscordEmbedColors.BREAKDOWN_ABNO_PAGE;
        const abnoType: string =
            abno.emotionSign > 0 ? "Awakening" : "Breakdown";
        return {
            title: abno.name,
            color: embedColor,
            image: {
                // TODO: less scuffed way to construct URL.
                // TODO: imagePath is currently prefixed with `/`. Revisit
                url: `https://${this.s3BucketName}.s3.amazonaws.com${abno.imagePath}`,
            },
            fields: [
                {
                    name: "Flavor text",
                    value: abno.flavorText,
                    inline: true,
                },
                {
                    name: "Effect",
                    value: abno.description,
                    inline: true,
                },
                {
                    name: "Bias",
                    value: String(abno.emotionRate),
                    inline: true,
                },
                {
                    name: "Type",
                    value: abnoType,
                    inline: true,
                },
                {
                    name: "Emotion level",
                    value: String(abno.emoLevel),
                    inline: true,
                },
                {
                    name: "Floor",
                    value: abno.floor,
                    inline: true,
                },
            ],
        };
    }

    // TODO: localize via _requestLocale
    public transformCombatPage(
        combat: DecoratedCombatPage,
        _requestLocale: Localization
    ): DiscordEmbed {
        const embedColor: number = this.rarityToColor(combat.rarity);
        return {
            title: combat.name,
            color: embedColor,
            image: {
                // TODO: less scuffed way to construct URL.
                // TODO: imagePath is currently prefixed with `/`. Revisit
                url: `https://${this.s3BucketName}.s3.amazonaws.com${combat.imagePath}`,
            },
            fields: [
                {
                    name: "Cost",
                    value: String(combat.cost),
                    inline: true,
                },
                {
                    name: "Range",
                    value: combat.range,
                    inline: true,
                },
                {
                    name: "Rarity",
                    value: combat.rarity,
                    inline: true,
                },
                {
                    name: "Description",
                    value: combat.description,
                    inline: true,
                },
                {
                    name: "Dice",
                    // TODO: implement
                    value: combat.description,
                    inline: true,
                },
            ],
        };
    }

    // TODO: localize via _requestLocale
    public transformDisambiguationPage(
        disambiguation: DisambiguationResults,
        _requestLocale: Localization
    ): DiscordEmbed {
        return {
            fields: [
                {
                    name: `**${disambiguation.query}** may refer to:`,
                    value: disambiguation.lookupResults
                        .map((lookupResult: LookupResult) => {
                            return lookupResult.query;
                        })
                        .map((str: string) => {
                            return ` > - ${str}`;
                        })
                        .join("\n"),
                    inline: true,
                },
            ],
        };
    }

    // TODO: localize via _requestLocale
    public noResultsFoundEmbed(
        query: string,
        _requestLocale: Localization
    ): DiscordEmbed {
        return {
            description: `*No results found for \`${query}\`.*`,
            fields: [],
        };
    }

    private rarityToColor(rarity: Rarity): number {
        switch (rarity) {
            case Rarity.PAPERBACK:
                return DiscordEmbedColors.PAPERBACK_RARITY;
            case Rarity.HARDCOVER:
                return DiscordEmbedColors.HARDCOVER_RARITY;
            case Rarity.LIMITED:
                return DiscordEmbedColors.LIMITED_RARITY;
            case Rarity.OBJET_D_ART:
                return DiscordEmbedColors.OBJET_D_ART_RARITY;
        }
    }
}

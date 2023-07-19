import { DecoratedAbnoPage, Localization } from "@ghoulean/ruina-common";
import { DiscordEmbedColors } from "../util/constants";
import { DiscordEmbed } from "../model/discord/discord_embed";

export class AbnoToEmbedTransformer {
    private readonly s3BucketName: string;

    constructor(s3BucketName: string) {
        this.s3BucketName = s3BucketName;
    }

    // TODO: strongly type embeds
    // TODO: localize via abno.locale
    public transform(abno: DecoratedAbnoPage): DiscordEmbed {
        const embedColor: number = abno.emotionSign > 0 ? DiscordEmbedColors.AWAKENING_ABNO_PAGE : DiscordEmbedColors.BREAKDOWN_ABNO_PAGE;
        const abnoType: string = abno.emotionSign > 0 ? "Awakening" : "Breakdown";
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
}

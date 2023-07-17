import { DecoratedAbnoPage, Localization } from "@ghoulean/ruina-common";

// TODO: implement
export class AbnoToEmbedTransformer {
    private readonly s3BucketName: string;

    constructor(s3BucketName: string) {
        this.s3BucketName = s3BucketName;
    }

    // TODO: strongly type embeds
    // TODO: localize
    public transform(abno: DecoratedAbnoPage, locale: Localization): any {
        return {
            type: "rich",
            title: abno.name,
            description: "",
            color: 0x380046,
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
                    value: abno.emotionRate,
                    inline: true,
                },
                {
                    name: "Type",
                    value: abno.emotionSign > 0 ? "Awakening" : "Breakdown",
                    inline: true,
                },
                {
                    name: "Emotion level",
                    value: abno.emoLevel,
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

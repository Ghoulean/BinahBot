import axios from "axios";
import { DiscordApplicationCommand } from "./models";

const BASE_DISCORD_URL = "https://discord.com/api/v10";

export interface DiscordAccessorProps {
    applicationId: string;
    botAuthToken: string;
}

export interface GlobalCommandList {
    commands: DiscordApplicationCommand[];
}

export class DiscordAccessor {
    private readonly applicationId: string;
    private readonly botAuthToken: string;
    private readonly authorizationHeader: { Authorization: string };

    constructor(props: DiscordAccessorProps) {
        this.applicationId = props.applicationId;
        this.botAuthToken = props.botAuthToken;
        this.authorizationHeader = {
            Authorization: `Bot ${this.botAuthToken}`,
        };
    }

    async getLiveGlobalCommands(): Promise<GlobalCommandList> {
        const list: DiscordApplicationCommand[] = [];
        const { data } = await axios.get(
            `${BASE_DISCORD_URL}/applications/${this.applicationId}/commands`,
            {
                headers: {
                    ...this.authorizationHeader,
                },
                params: {
                    with_localizations: "true",
                },
            }
        );
        for (const d of data) {
            list.push(d as unknown as DiscordApplicationCommand);
        }
        return {
            commands: list,
        };
    }

    async writeGlobalCommands(commands: GlobalCommandList): Promise<GlobalCommandList> {
        try {
            const { status, data } = await axios.put(
                `${BASE_DISCORD_URL}/applications/${this.applicationId}/commands`,
                JSON.stringify(commands.commands),
                {
                    headers: {
                        ...this.authorizationHeader,
                        "Content-Type": "application/json",
                    },
                }
            )
            if (status === 200) {
                console.log(
                    "Commands (over)written successfully"
                );
            }
            return data
        } catch (e: any) {
            if (e.response) {
                console.log("data: ", JSON.stringify(e.response.data));
                console.log("status: ", e.response.status);
                console.log("headers: ", e.response.headers);
            }
            return { commands: []}
        }
    }

    async deleteGlobalCommandById(id: string): Promise<boolean> {
        const { status } = await axios.delete(
            `${BASE_DISCORD_URL}/applications/${this.applicationId}/commands/${id}`,
            {
                headers: {
                    ...this.authorizationHeader,
                },
            }
        );
        console.log(`Deleted ${id} with status ${status}`);
        return true;
    }

    async uploadAvatar(avatar_png_base64: string): Promise<boolean> {
        const { data } = await axios.patch(
            `${BASE_DISCORD_URL}/users/@me`,
            {
                avatar: `data:image/png;base64,${avatar_png_base64}`
            },
            {
                headers: {
                    ...this.authorizationHeader,
                },
            }
        );
        console.log(`Successfully uploaded avatar: ${JSON.stringify(data)}`);
        return true;
    }
}

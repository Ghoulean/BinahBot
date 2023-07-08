import axios from 'axios';
import { commands } from './commands.json';

const BASE_DISCORD_URL = "https://discord.com/api/v10"; // /applications/1123461446198636634/commands?with_localizations=true"

export interface DiscordAccessorProps {
    applicationId: string;
    botAuthToken: string;
}

// https://discord.com/developers/docs/reference#locales
export interface GlobalCommand {
    id: string,
    names: {[key: string]: string};
    type: number;
    descriptions: {[key: string]: string};
}

export interface GlobalCommandList {
    commands: GlobalCommand[];
}

interface DesiredGlobalCommand {
    names: {[key: string]: string};
    type: number;
    descriptions: {[key: string]: string};
}

export const DEFAULT_LOCALE_CODE = "DEFAULT";

export class DiscordAccessor {
    private readonly applicationId: string;
    private readonly botAuthToken: string;
    private readonly authorizationHeader: {"Authorization": string};
    private readonly desiredCommands: DesiredGlobalCommand[];

    constructor(props: DiscordAccessorProps) {
        this.applicationId = props.applicationId;
        this.botAuthToken = props.botAuthToken;
        this.authorizationHeader = {
            "Authorization": `Bot ${this.botAuthToken}`
        };
        this.desiredCommands = commands;
    }

    async getLiveGlobalCommands(): Promise<GlobalCommandList> {
        const list: GlobalCommand[] = [];
        try {
            const { data } = await axios.get(`${BASE_DISCORD_URL}/applications/${this.applicationId}/commands`, {
                headers: {
                    ...this.authorizationHeader
                },
                params: {
                    "with_localizations": "true"
                }
            });
            for (const d of data) {
                list.push(this.parseGlobalCommand(d));
            }
        } catch (e) {
            throw e;
        }
        return {
            "commands": list
        };
    }

    async writeGlobalCommands(): Promise<GlobalCommandList> {
        const list: GlobalCommand[] = [];
        for (const desiredCommand of this.desiredCommands) {
            const formBody = this.transformIntoFormBody(desiredCommand);
            console.log(formBody);
            try {
                const { status, data } = await axios.post(`${BASE_DISCORD_URL}/applications/${this.applicationId}/commands`, 
                formBody,
                {
                    headers: {
                        ...this.authorizationHeader,
                        "Content-Type": "application/json"
                    }
                });
                if (status === 201) {
                    console.log("New command created: " + desiredCommand.names["en-US"]);
                } else if (status === 200) {
                    console.log("Existing command overwritten with new one: " + desiredCommand.names["en-US"]);
                }

                list.push(this.parseGlobalCommand(data));
            } catch (e) {
                throw e;
            }
        }
        return {
            "commands": list
        };
    }

    async deleteGlobalCommandById(id: string): Promise<boolean> {
        try {
            const { status } = await axios.delete(`${BASE_DISCORD_URL}/applications/${this.applicationId}/commands/${id}`,
            {
                headers: {
                    ...this.authorizationHeader
                }
            });
            console.log(`Deleted ${id} with status ${status}`);
        } catch (e) {
            throw e;
        }
        return true;
    }

    private transformIntoFormBody(desiredCommand: DesiredGlobalCommand): any {
        return {
            "dm_permission": true,
            "nsfw": false,
            "name": desiredCommand.names["en-US"],
            "name_localizations": desiredCommand.names,
            "description": desiredCommand.descriptions["en-US"],
            "description_localizations": desiredCommand.descriptions,
            "type": desiredCommand.type,
        }
    }

    private parseGlobalCommand(blob: any): GlobalCommand {
        return {
            id: blob.id!,
            names: {
                DEFAULT_LOCALE_CODE: blob.name,
                ...blob.name_localizations,
            },
            type: blob.type!,
            descriptions: {
                DEFAULT_LOCALE_CODE: blob.description,
                ...blob.description_localizations
            }
        };
    }
}

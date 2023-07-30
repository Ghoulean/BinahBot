import { Chapter, Localization } from "@ghoulean/ruina-common";

export type Request = {
    command: string;
    commandArgs: RequestCommandArgs;
    interactionToken: string;
    locale: Localization;
    chapter: Chapter;
    autocomplete?: boolean;
};

export type RequestCommandArgs = {
    [key: string]: string | number | boolean
};

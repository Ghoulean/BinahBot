import { LookupResult } from "@ghoulean/ruina-common";
import { DataAccessor } from "../accessor/data_accessor";
import { CommandResult } from "../model/command_result";
import { Request } from "../model/request";

// TODO: think about where these constants go
const QUERY_COMMAND_ARG = "query";
const AUTOCOMPLETE_LIMIT = 5;

// TODO: consider interfacing autocomplete function
export class LorAutocomplete {
    private readonly dataAccessor: DataAccessor;

    constructor(dataAccessor: DataAccessor) {
        this.dataAccessor = dataAccessor;
    }

    public autocomplete(request: Request): CommandResult {
        const query: string = request.commandArgs[QUERY_COMMAND_ARG] as string;
        const autocompleteResults: string[] = this.dataAccessor
            .autocomplete(query)
            .slice(0, AUTOCOMPLETE_LIMIT)
            .map((str: string) => {
                return this.dataAccessor.lookup(str, request.locale);
            })
            .map((lookupResult: LookupResult) => {
                return lookupResult.displayQuery ?? lookupResult.query;
            });

        return {
            success: true,
            payload: autocompleteResults,
        };
    }
}

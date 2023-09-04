import { LorAutocomplete } from "../command/lor_autocomplete";
import { AutocompleteResult, CommandResult } from "../model/command_result";
import { Request } from "../model/request";

export class AutocompleteRouter {
    private readonly lorAutocomplete: LorAutocomplete;

    // TODO: figure out a way to put this in a prop object
    // Can't do so right now due to inability to mock dependencies
    // (Seems to be a jest.Mock + Typescript limitation?)
    constructor(lorAutocomplete: LorAutocomplete) {
        this.lorAutocomplete = lorAutocomplete;
    }

    public routeRequest(request: Request): AutocompleteResult {
        console.log(
            `RequestRouter: received request ${JSON.stringify(request)}`
        );
        switch (request.command) {
            // TODO: consider moving to enum
            case "lor":
                return this.lorAutocomplete.autocomplete(request);
            default:
                console.log(
                    `RequestRouter: Did not find route for ${request.command}`
                );
                return {
                    success: false,
                    error: `Command for ${request.command} not found`,
                };
        }
    }
}

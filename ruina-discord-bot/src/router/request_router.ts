import { LorAutocomplete } from "../command/lor_autocomplete";
import { LorCommand } from "../command/lor_command";
import { CommandResult } from "../model/command_result";
import { Request } from "../model/request";

export class RequestRouter {
    private readonly lorCommand: LorCommand;
    private readonly lorAutocomplete: LorAutocomplete;

    constructor(lorCommand: LorCommand, lorAutocomplete: LorAutocomplete) {
        this.lorCommand = lorCommand;
        this.lorAutocomplete = lorAutocomplete;
    }

    public routeRequest(request: Request): CommandResult {
        console.log(`RequestRouter: received request ${JSON.stringify(request)}`);
        switch (request.command) {
            // TODO: consider moving to enum
            case "lor":
                if (request.autocomplete) {
                    console.log(
                        `RequestRouter: Routing request to LOR autocomplete`
                    );
                    return this.lorAutocomplete.autocomplete(request);
                } else {
                    console.log(
                        `RequestRouter: Routing request to LOR command`
                    );
                    return this.lorCommand.invoke(request);
                }
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

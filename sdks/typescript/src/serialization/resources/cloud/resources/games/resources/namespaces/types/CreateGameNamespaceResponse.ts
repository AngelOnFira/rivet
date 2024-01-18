/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../../../../../..";
import * as Rivet from "../../../../../../../../api";
import * as core from "../../../../../../../../core";

export const CreateGameNamespaceResponse: core.serialization.ObjectSchema<
    serializers.cloud.games.namespaces.CreateGameNamespaceResponse.Raw,
    Rivet.cloud.games.namespaces.CreateGameNamespaceResponse
> = core.serialization.object({
    namespaceId: core.serialization.property("namespace_id", core.serialization.string()),
});

export declare namespace CreateGameNamespaceResponse {
    interface Raw {
        namespace_id: string;
    }
}

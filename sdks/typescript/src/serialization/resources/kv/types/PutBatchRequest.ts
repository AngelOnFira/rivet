/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../..";
import * as Rivet from "../../../../api";
import * as core from "../../../../core";

export const PutBatchRequest: core.serialization.ObjectSchema<
    serializers.kv.PutBatchRequest.Raw,
    Rivet.kv.PutBatchRequest
> = core.serialization.object({
    namespaceId: core.serialization.property("namespace_id", core.serialization.string().optional()),
    entries: core.serialization.list(core.serialization.lazyObject(async () => (await import("../../..")).kv.PutEntry)),
});

export declare namespace PutBatchRequest {
    interface Raw {
        namespace_id?: string | null;
        entries: serializers.kv.PutEntry.Raw[];
    }
}

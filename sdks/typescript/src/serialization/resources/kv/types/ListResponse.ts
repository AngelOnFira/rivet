/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../..";
import * as Rivet from "../../../../api";
import * as core from "../../../../core";

export const ListResponse: core.serialization.ObjectSchema<serializers.kv.ListResponse.Raw, Rivet.kv.ListResponse> =
    core.serialization.object({
        entries: core.serialization.list(
            core.serialization.lazyObject(async () => (await import("../../..")).kv.Entry)
        ),
    });

export declare namespace ListResponse {
    interface Raw {
        entries: serializers.kv.Entry.Raw[];
    }
}

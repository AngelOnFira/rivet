/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../..";
import * as Rivet from "../../../../api";
import * as core from "../../../../core";

export const ListFriendsResponse: core.serialization.ObjectSchema<
    serializers.identity.ListFriendsResponse.Raw,
    Rivet.identity.ListFriendsResponse
> = core.serialization.object({
    identities: core.serialization.list(
        core.serialization.lazyObject(async () => (await import("../../..")).identity.Handle)
    ),
    anchor: core.serialization.string().optional(),
    watch: core.serialization.lazyObject(async () => (await import("../../..")).WatchResponse),
});

export declare namespace ListFriendsResponse {
    interface Raw {
        identities: serializers.identity.Handle.Raw[];
        anchor?: string | null;
        watch: serializers.WatchResponse.Raw;
    }
}

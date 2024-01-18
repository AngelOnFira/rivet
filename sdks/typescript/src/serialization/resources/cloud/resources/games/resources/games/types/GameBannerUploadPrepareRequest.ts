/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../../../../../..";
import * as Rivet from "../../../../../../../../api";
import * as core from "../../../../../../../../core";

export const GameBannerUploadPrepareRequest: core.serialization.ObjectSchema<
    serializers.cloud.games.GameBannerUploadPrepareRequest.Raw,
    Rivet.cloud.games.GameBannerUploadPrepareRequest
> = core.serialization.object({
    path: core.serialization.string(),
    mime: core.serialization.string().optional(),
    contentLength: core.serialization.property("content_length", core.serialization.number()),
});

export declare namespace GameBannerUploadPrepareRequest {
    interface Raw {
        path: string;
        mime?: string | null;
        content_length: number;
    }
}

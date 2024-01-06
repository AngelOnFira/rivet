/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../../../..";
import * as Rivet from "../../../../../../api";
import * as core from "../../../../../../core";

export const CdnNamespaceDomain: core.serialization.ObjectSchema<
    serializers.cloud.CdnNamespaceDomain.Raw,
    Rivet.cloud.CdnNamespaceDomain
> = core.serialization.object({
    domain: core.serialization.string(),
    createTs: core.serialization.property("create_ts", core.serialization.date()),
    verificationStatus: core.serialization.property(
        "verification_status",
        core.serialization.lazy(async () => (await import("../../../../..")).cloud.CdnNamespaceDomainVerificationStatus)
    ),
    verificationMethod: core.serialization.property(
        "verification_method",
        core.serialization.lazyObject(
            async () => (await import("../../../../..")).cloud.CdnNamespaceDomainVerificationMethod
        )
    ),
    verificationErrors: core.serialization.property(
        "verification_errors",
        core.serialization.list(core.serialization.string())
    ),
});

export declare namespace CdnNamespaceDomain {
    interface Raw {
        domain: string;
        create_ts: string;
        verification_status: serializers.cloud.CdnNamespaceDomainVerificationStatus.Raw;
        verification_method: serializers.cloud.CdnNamespaceDomainVerificationMethod.Raw;
        verification_errors: string[];
    }
}
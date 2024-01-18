/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../..";
import * as Rivet from "../../../../api";
import * as core from "../../../../core";

export const BootstrapDomains: core.serialization.ObjectSchema<
    serializers.cloud.BootstrapDomains.Raw,
    Rivet.cloud.BootstrapDomains
> = core.serialization.object({
    main: core.serialization.string(),
    cdn: core.serialization.string(),
    job: core.serialization.string(),
});

export declare namespace BootstrapDomains {
    interface Raw {
        main: string;
        cdn: string;
        job: string;
    }
}

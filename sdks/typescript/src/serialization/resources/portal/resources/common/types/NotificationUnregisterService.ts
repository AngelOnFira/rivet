/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../../../..";
import * as Rivet from "../../../../../../api";
import * as core from "../../../../../../core";

export const NotificationUnregisterService: core.serialization.Schema<
    serializers.portal.NotificationUnregisterService.Raw,
    Rivet.portal.NotificationUnregisterService
> = core.serialization.enum_(["firebase"]);

export declare namespace NotificationUnregisterService {
    type Raw = "firebase";
}

/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../..";
import * as Rivet from "../../../../api";
import * as core from "../../../../core";

export const Timestamp: core.serialization.Schema<serializers.Timestamp.Raw, Rivet.Timestamp> =
    core.serialization.string();

export declare namespace Timestamp {
    type Raw = string;
}
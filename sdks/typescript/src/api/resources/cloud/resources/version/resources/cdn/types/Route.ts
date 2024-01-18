/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as Rivet from "../../../../../../..";

export interface Route {
    glob: string;
    /** Unsigned 32 bit integer. */
    priority: number;
    /** Multiple CDN version middleware. */
    middlewares: Rivet.cloud.version.cdn.Middleware[];
}

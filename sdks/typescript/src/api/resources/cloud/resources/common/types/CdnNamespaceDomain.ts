/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as Rivet from "../../../../..";

/**
 * A CDN domain for a given namespace.
 */
export interface CdnNamespaceDomain {
    /** A valid domain name (no protocol). */
    domain: string;
    /** RFC3339 timestamp. */
    createTs: Date;
    verificationStatus: Rivet.cloud.CdnNamespaceDomainVerificationStatus;
    verificationMethod: Rivet.cloud.CdnNamespaceDomainVerificationMethod;
    verificationErrors: string[];
}
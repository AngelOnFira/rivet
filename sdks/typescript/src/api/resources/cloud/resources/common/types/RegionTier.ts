/**
 * This file was auto-generated by Fern from our API Definition.
 */

/**
 * A region server tier.
 */
export interface RegionTier {
    /** A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short. */
    tierNameId: string;
    /** Together with the denominator, denotes the portion of the CPU a given server uses. */
    rivetCoresNumerator: number;
    /** Together with the numerator, denotes the portion of the CPU a given server uses. */
    rivetCoresDenominator: number;
    /** CPU frequency (MHz). */
    cpu: number;
    /** Allocated memory (MB). */
    memory: number;
    /** Allocated disk space (MB). */
    disk: number;
    /** Internet bandwidth (MB). */
    bandwidth: number;
    /**
     * **Deprecated**
     * Price billed for every second this server is running (in quadrillionth USD, 1,000,000,000,000 = $1.00).
     */
    pricePerSecond: number;
}

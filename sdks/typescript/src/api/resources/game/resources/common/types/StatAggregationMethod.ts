/**
 * This file was auto-generated by Fern from our API Definition.
 */

/**
 * A value denoting the aggregation method of a game statistic.
 */
export type StatAggregationMethod =
    /**
     * Summation aggregation. */
    | "sum"
    /**
     * Average aggregation. */
    | "average"
    /**
     * Minimum value aggregation. */
    | "min"
    /**
     * Maximum value aggregation. */
    | "max";

export const StatAggregationMethod = {
    Sum: "sum",
    Average: "average",
    Min: "min",
    Max: "max",
} as const;

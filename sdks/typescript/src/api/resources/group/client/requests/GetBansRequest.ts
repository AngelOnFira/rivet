/**
 * This file was auto-generated by Fern from our API Definition.
 */

export interface GetBansRequest {
    /**
     * The pagination anchor. Set to the returned anchor of this endpoint to receive the next set of items.
     */
    anchor?: string;
    /**
     * Amount of bans to return.
     */
    count?: number;
    /**
     * A query parameter denoting the requests watch index.
     */
    watchIndex?: string;
}

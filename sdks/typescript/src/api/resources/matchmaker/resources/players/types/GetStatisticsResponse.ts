/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as Rivet from "../../../../..";

export interface GetStatisticsResponse {
    playerCount: number;
    gameModes: Record<Rivet.Identifier, Rivet.matchmaker.GameModeStatistics>;
}

/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as Rivet from "../../../../..";

/**
 * The game an identity is currently participating in.
 */
export interface GameActivity {
    game: Rivet.game.Handle;
    /** A short activity message about the current game activity. */
    message: string;
    /** JSON data seen by anyone. */
    publicMetadata?: unknown;
    /** JSON data seen only by the given identity and their mutual followers. */
    mutualMetadata?: unknown;
}

// Phosphor icons (regular), inlined at build time.
import arrowRight from "@phosphor-icons/core/assets/regular/arrow-right.svg?raw";
import arrows from "@phosphor-icons/core/assets/regular/arrows-clockwise.svg?raw";
import download from "@phosphor-icons/core/assets/regular/download-simple.svg?raw";
import folders from "@phosphor-icons/core/assets/regular/folders.svg?raw";
import github from "@phosphor-icons/core/assets/regular/github-logo.svg?raw";
import lock from "@phosphor-icons/core/assets/regular/lock.svg?raw";
import puzzle from "@phosphor-icons/core/assets/regular/puzzle-piece.svg?raw";
import shield from "@phosphor-icons/core/assets/regular/shield-check.svg?raw";
import windows from "@phosphor-icons/core/assets/regular/windows-logo.svg?raw";
import caret from "@phosphor-icons/core/assets/regular/caret-down.svg?raw";

export const icons = { arrowRight, arrows, download, folders, github, lock, puzzle, shield, windows, caret };
export type IconName = keyof typeof icons;

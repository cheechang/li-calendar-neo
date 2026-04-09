import { isWindows } from '../utils/platform.ts';

export const WINDOW_RADIUS = isWindows ? 8 : 16;
export const WINDOW_RADIUS_CLIP_PATH = `inset(0 round ${WINDOW_RADIUS}px)`;

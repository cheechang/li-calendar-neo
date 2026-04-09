import { useEffect } from 'react';
import { WINDOW_RADIUS, WINDOW_RADIUS_CLIP_PATH } from '../constants/window.ts';

export const useWindowCornerMask = (): void => {
  useEffect(() => {
    const html = document.documentElement;
    const body = document.body;
    const root = document.getElementById('root');
    if (!root) return;

    const previous = {
      htmlBackground: html.style.background,
      bodyBackground: body.style.background,
      rootBackground: root.style.background,
      htmlOverflow: html.style.overflow,
      bodyOverflow: body.style.overflow,
      rootOverflow: root.style.overflow,
      htmlBorderRadius: html.style.borderRadius,
      bodyBorderRadius: body.style.borderRadius,
      rootBorderRadius: root.style.borderRadius,
      htmlClipPath: html.style.clipPath,
      bodyClipPath: body.style.clipPath,
      rootClipPath: root.style.clipPath,
    };

    html.style.background = 'transparent';
    body.style.background = 'transparent';
    html.style.overflow = 'hidden';
    body.style.overflow = 'hidden';
    html.style.borderRadius = `${WINDOW_RADIUS}px`;
    body.style.borderRadius = `${WINDOW_RADIUS}px`;
    html.style.clipPath = WINDOW_RADIUS_CLIP_PATH;
    body.style.clipPath = WINDOW_RADIUS_CLIP_PATH;

    root.style.background = 'transparent';
    root.style.overflow = 'hidden';
    root.style.borderRadius = `${WINDOW_RADIUS}px`;
    root.style.clipPath = WINDOW_RADIUS_CLIP_PATH;

    return () => {
      html.style.background = previous.htmlBackground;
      body.style.background = previous.bodyBackground;
      html.style.overflow = previous.htmlOverflow;
      body.style.overflow = previous.bodyOverflow;
      html.style.borderRadius = previous.htmlBorderRadius;
      body.style.borderRadius = previous.bodyBorderRadius;
      html.style.clipPath = previous.htmlClipPath;
      body.style.clipPath = previous.bodyClipPath;

      root.style.background = previous.rootBackground;
      root.style.overflow = previous.rootOverflow;
      root.style.borderRadius = previous.rootBorderRadius;
      root.style.clipPath = previous.rootClipPath;
    };
  }, []);
};
